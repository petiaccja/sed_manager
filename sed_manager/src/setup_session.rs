//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::sync::Arc;

use sed_device::StorageDevice;
use sed_packet::MaxBytes;
use sed_spec::methods::MethodStatus;
use sed_spec::objects::{Authority, AuthorityRef, AuthorityRefExt as _, CPinRef, CPinRefExt, SecurityProviderRef};
use sed_spec::ssc::Ssc;
use sed_spec::types::AuthMethod;
use sed_spec::uid_registry::common::admin::c_pin::{MSID as C_PIN_MSID, SID as C_PIN_SID};
use sed_spec::uid_registry::core::shared::authority::SID;
use sed_spec::uid_registry::core::shared::table_id;
use sed_spec::uid_registry::psid::admin::authority::PSID;
use sed_spec::uid_registry::{common, enterprise, kpio, psid};
use sed_tper::{Error as TperError, Tper, TperHandle};
use tracing::instrument;

use crate::DeviceSpec;
use crate::Error::{self, NotSupported};

/// Performs simple device setup and information retrieval operations.
///
/// The TPer sessions opened by this object are short-lived, and closed
/// immediately once they data has been read of written. From the perspective
/// of callers, this can still be regarded as a persistent session, as it will
/// anyways count towards the limit on the number of sessions the device
/// supports, though only intermittently.
#[derive(Debug)]
pub struct SetupSession {
    storage_device: Arc<dyn StorageDevice>,
    spec: DeviceSpec,
    tper: TperHandle,
}

impl SetupSession {
    /// Open a session on the SSC given by `spec`.
    pub fn new(tper: &Tper, spec: DeviceSpec) -> Self {
        Self { storage_device: tper.storage_device(), tper: tper.handle(), spec }
    }

    pub fn spec(&self) -> &DeviceSpec {
        &self.spec
    }

    /// Take ownership of the device by changing the SID password.
    ///
    /// # Errors
    ///
    /// To change the SID password, we first have to authenticate the SID
    /// authority using its initial (MSID) password. If ownership has already
    /// been taken, the MSID password won't work, and an error is returned.
    ///
    /// The method may also fail due to any common Tper RPC errors.
    #[instrument(level = "info", skip(self, new_sid_password), ret, err)]
    pub async fn take_owneship(&self, new_sid_password: MaxBytes<32>) -> Result<(), Error> {
        // Get the MSID password.
        let initial_password = self
            .tper
            .start_session(common::admin::sp::ADMIN, None, None)
            .await?
            .with(async |session| session.get_field(C_PIN_MSID.pin()).await)
            .await?;

        // Change the SID password to the new one.
        self.tper
            .start_session(common::admin::sp::ADMIN, Some(SID), Some(initial_password.clone()))
            .await
            .map_err(|err| match err {
                TperError::MethodCallFailed(MethodStatus::NotAuthorized) => Error::AlreadyOwned,
                err => err.into(),
            })?
            .with(async |session| session.set_field(C_PIN_SID.pin(), new_sid_password.clone()).await)
            .await?;

        // For Enterprise, change the password of EraseMaster and all BandMasters
        // to the SID PIN instead of their default MSID PIN.
        if self.spec.ssc() == Ssc::Enterprise {
            let authorities = self
                .tper
                .start_session(enterprise::admin::sp::LOCKING, None, None)
                .await?
                .with(async |session| session.next(None::<AuthorityRef>, None).await)
                .await?;
            let authorities = authorities.into_iter().filter(|authority| {
                *authority == enterprise::locking::authority::ERASE_MASTER
                    || enterprise::locking::authority::BAND_MASTER.contains(*authority)
            });
            for authority in authorities {
                self.change_password(
                    enterprise::admin::sp::LOCKING,
                    authority,
                    initial_password.clone(),
                    new_sid_password.clone(),
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Activate the locking or key per I/O SPs.
    ///
    /// # Errors
    ///
    /// The session as the SID authority is used to activate the locking or KPIO SP.
    /// If the discovery indicates that the secondary SP is already active,
    /// [`Error::AlreadyActivated`] is returned.
    ///
    /// The method may also fail due to any common Tper RPC errors.
    #[instrument(level = "info", skip(self, sid_password), ret, err)]
    pub async fn activate_secondary_sp(&self, sid_password: MaxBytes<32>) -> Result<(), Error> {
        let ssc = self.spec().ssc();

        let discovery = Tper::discover(&*self.storage_device).await?;

        match discovery.is_secondary_sp_enabled() {
            Some(true) => Err(Error::AlreadyActivated),
            Some(false) => self
                .tper
                .start_session(common::admin::sp::ADMIN, Some(SID), Some(sid_password))
                .await?
                .with(async |session| session.activate(ssc.secondary_sp()).await)
                .await
                .map_err(Into::into),
            None => Err(Error::IncompatibleSsc),
        }
    }

    /// Revert the whole device to its original factory state.
    ///
    /// This method requires a login to the Admin SP as SID or PSID. Both the
    /// admin and the secondary (locking or KPIO) SP will be reverted to their
    /// original factory state.
    ///
    /// # Parameters
    ///
    /// - `authority`: this may be the SID or the PSID authority.
    /// - `password`: the password of the `authority`.
    #[instrument(level = "info", skip(self, password), ret, err)]
    pub async fn revert_tper(&self, authority: AuthorityRef, password: MaxBytes<32>) -> Result<(), Error> {
        let is_action_supported = (authority == PSID && self.is_psid_supported().await?)
            || (authority != PSID && self.spec.ssc().revert_by_sid_supported());
        if is_action_supported {
            let session = self.tper.start_session(common::admin::sp::ADMIN, Some(authority), Some(password)).await?;
            session
                .with(async |session| session.revert(common::admin::sp::ADMIN).await)
                .await
                .map_err(Into::into)
        } else {
            Err(Error::NotSupported)
        }
    }

    /// Revert the secondary SP to its original factory state.
    ///
    /// This method requires a login to the Admin SP as SID.
    /// Only the secondary SP will be reverted, the Admin SP is unaffected.
    #[instrument(level = "info", skip(self, sid_password), ret, err)]
    pub async fn revert_secondary_sp(&self, sid_password: MaxBytes<32>) -> Result<(), Error> {
        if self.spec.ssc().revert_by_sid_supported() {
            self.tper
                .start_session(common::admin::sp::ADMIN, Some(SID), Some(sid_password))
                .await?
                .with(async |session| session.revert(self.spec.ssc().secondary_sp()).await)
                .await
                .map_err(Into::into)
        } else {
            Err(NotSupported)
        }
    }

    /// Revert the secondary SP to its original factory state.
    ///
    /// This method requires a login to the secondary SP as one of the admins.
    /// Only the secondary SP will be reverted, the Admin SP is unaffected.
    #[instrument(level = "info", skip(self, password), ret, err)]
    pub async fn revert_secondary_sp_ex(
        &self,
        admin: AuthorityRef,
        password: MaxBytes<32>,
        keep_global_range_key: Option<bool>,
    ) -> Result<(), Error> {
        if self.spec.ssc().revert_sp_supported() {
            let session = self.tper.start_session(self.spec.ssc().secondary_sp(), Some(admin), Some(password)).await?;
            match session.revert_sp(keep_global_range_key).await {
                Ok(()) => Ok(()),
                Err((session, err)) => {
                    let _ = session.close().await;
                    Err(err.into())
                }
            }
        } else {
            Err(Error::NotSupported)
        }
    }

    /// Change the password of the `authority` on the `sp`.
    ///
    /// The password is changed by starting a session on the `authority` using
    /// its current password, and then writing the new password to the C_PIN
    /// table. In some cases, authorities may change the password of another
    /// authority, but this is not leveraged here.
    #[instrument(level = "info", skip(self, current_password, new_password), ret, err)]
    pub async fn change_password(
        &self,
        sp: SecurityProviderRef,
        authority: AuthorityRef,
        current_password: MaxBytes<32>,
        new_password: MaxBytes<32>,
    ) -> Result<(), Error> {
        let session = self.tper.start_session(sp, Some(authority), Some(current_password)).await?;
        session
            .with(async |session| {
                let credential = session.get_field(authority.credential()).await?;
                let pin_ref = CPinRef::try_from(credential).map_err(|_| Error::NotAPasswordAuthority)?;
                session.set_field(pin_ref.pin(), new_password).await?;
                Ok(())
            })
            .await
    }

    /// List the authorities on the `sp`.
    ///
    /// This function will also attempt to retrieve all columns of the
    /// authorities. Since it's authenticated only as the `Anybody` authority,
    /// the returned columns may be incomplete, as `Anybody` may not have
    /// access.
    #[instrument(level = "info", skip(self), ret, err)]
    pub async fn list_authorities(&self, sp: SecurityProviderRef) -> Result<Vec<Authority>, Error> {
        match (self.spec.ssc(), sp) {
            (Ssc::Enterprise, enterprise::admin::sp::ADMIN) => {
                let has_psid = self.is_psid_supported().await?;
                let authority_refs = enterprise_admin_authorities(has_psid);
                Ok(authority_refs
                    .into_iter()
                    .map(|authority_ref| append_authority_uid_and_op(authority_ref, Default::default()))
                    .collect())
            }
            _ => {
                self.tper
                    .start_session(sp, None, None)
                    .await?
                    .with(async |session| {
                        let authority_refs = session.next::<{ table_id::AUTHORITY.to_u64() }>(None, None).await?;
                        let mut authorities = Vec::new();
                        for authority_ref in authority_refs {
                            let authority = session.get_object(authority_ref, ..).await?;
                            authorities.push(append_authority_uid_and_op(authority_ref, authority));
                        }
                        Ok(authorities)
                    })
                    .await
            }
        }
    }

    /// Checks whether the PSID feature set is supported.
    #[instrument(level = "info", skip(self), ret, err)]
    pub async fn is_psid_supported(&self) -> Result<bool, Error> {
        if self.spec.ssc().psid_mandatory() {
            // We should spare starting an extra session; non-compliant devices
            // will error out on PSID operations.
            Ok(true)
        } else {
            // Check if the PSID authority exists by getting one of its fields
            // readable by `Anybody`.
            let session = self.tper.start_session(common::admin::sp::ADMIN, None, None).await?;
            let enabled = session.with(async |session| session.get_field(PSID.enabled()).await).await;
            match enabled.map_err(Error::check_missing) {
                Ok(_) => Ok(true),
                Err(Error::NotSupported) => Ok(false),
                Err(err) => Err(err),
            }
        }
    }
}

pub fn append_authority_uid_and_op(authority_ref: AuthorityRef, authority: Authority) -> Authority {
    use common::admin::authority as common_admin;
    use common::locking::authority as common_locking;
    use enterprise::admin::authority as enterprise_admin;
    use enterprise::locking::authority as enterprise_locking;
    use kpio::key_per_io::authority as kpio_kpio;
    use psid::admin::authority as psid_admin;
    use sed_spec::uid_registry::core::shared::authority as core_shared;

    let operation = match authority_ref {
        // Core.
        core_shared::ANYBODY => Some(AuthMethod::None),
        core_shared::SID => Some(AuthMethod::Password),
        core_shared::MAKERS => Some(AuthMethod::None),
        // Shared among SSCs.
        common_admin::ADMINS => Some(AuthMethod::None),
        // PSID
        psid_admin::PSID => Some(AuthMethod::Password),
        // Enterprise
        enterprise_admin::MAKER_SYM_K => Some(AuthMethod::SymK),
        _ if enterprise_locking::BAND_MASTER.contains(authority_ref) => Some(AuthMethod::Password),
        enterprise_locking::ERASE_MASTER => Some(AuthMethod::Password),
        enterprise_locking::BAND_MASTERS => Some(AuthMethod::None),
        // Opal family and Key per I/O
        common_locking::USERS => Some(AuthMethod::None),
        _ if common_admin::ADMIN.contains(authority_ref)
            || common_locking::ADMIN.contains(authority_ref)
            || common_locking::USER.contains(authority_ref)
            || kpio_kpio::ADMIN.contains(authority_ref) =>
        {
            Some(AuthMethod::Password)
        }
        // Unknown
        _ => None,
    };

    Authority {
        uid: authority.uid.or(Some(authority_ref)),
        operation: authority.operation.or(operation),
        ..authority
    }
}

/// Return the list of authorities on the Enterprise Admin SP. `Next` is not
/// authorized on the SP, so we must use the preconfiguration.
fn enterprise_admin_authorities(has_psid: bool) -> Vec<AuthorityRef> {
    use enterprise::admin::authority::{ANYBODY, MAKER_SYM_K, MAKERS, SID};
    [ANYBODY, SID, MAKERS, MAKER_SYM_K].into_iter().chain(has_psid.then_some(PSID)).collect()
}
