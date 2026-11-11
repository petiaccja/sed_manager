//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_packet::{ObjectUid, Uid};
use sed_spec_macros::{DetokenizeStruct, FieldList, Object, TokenizeField, TokenizeStruct};

use crate::objects::{AuthorityRef, LogListRef};
use crate::types::{AuthMethod, Date, HashProtocol, LogSelect, MessagingType};
use crate::uid_registry::core::shared::table_id;
use crate::uid_registry::{core, enterprise, opal_2, psid};

#[derive(Debug, Clone, Default, PartialEq, Eq, Object, TokenizeStruct, DetokenizeStruct, FieldList, TokenizeField)]
#[object(table=table_id::AUTHORITY)]
pub struct Authority {
    pub uid: Option<AuthorityRef>,
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub is_class: Option<bool>,
    pub class: Option<AuthorityRef>,
    pub enabled: Option<bool>,
    pub secure: Option<MessagingType>,
    pub hash_and_sign: Option<HashProtocol>,
    pub present_certificate: Option<bool>,
    pub operation: Option<AuthMethod>,
    pub credential: Option<Uid>,
    pub response_sign: Option<AuthorityRef>,
    pub response_exch: Option<AuthorityRef>,
    pub clock_start: Option<Date>,
    pub clock_end: Option<Date>,
    pub limit: Option<u32>,
    pub uses: Option<u32>,
    pub log: Option<LogSelect>,
    pub log_to: Option<LogListRef>,
}

impl ObjectUid for Authority {
    fn uid(&self) -> Option<Self::Ref> {
        self.uid
    }

    fn set_uid(&mut self, uid: Option<Self::Ref>) {
        self.uid = uid;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuthorityKind {
    /// The `Anybody` authority. Applicable to all SSCs.
    Anybody,
    /// Any of the known class authorities.
    Class(AuthorityClass),
    /// The SID authority on the Admin SP. Applicable to all SSCs.
    Sid,
    /// The PSID authority on the Admin SP. Applicable to SSCs that have the PSID
    /// feature set.
    Psid,
    /// An admin authority on the Admin SP, with its 0-based index. Applicable
    /// to the Opal family and KPIO SSCs.
    AdminAdm(usize),
    /// An admin authority on the Locking or KPIO SP, with its 0-based index.
    /// Applicable to the Opal family and KPIO SSCs.
    AdminSec(usize),
    /// A User authority on the Locking SP, with its 0-based index. Applicable
    /// to the Opal family SSCs.
    User(usize),
    /// An Enterprise SSCs BandMaster authority.
    BandMaster(usize),
    /// The Enterprise SSC EraseMaster authority.
    EraseMaster,
    /// Authorities that could not be categorized.
    Other(AuthorityRef),
}

impl AuthorityKind {
    /// Determines the class of the authority based on its UID.
    ///
    /// If the authority belongs to the `Admins`, `Users`, or `BandMasters`
    /// classes, that is returned. This is derived directly from the authority's
    /// UID, without the need to `Get` the authority object's `Class` field.
    ///
    /// For other authorities, `None` is returned even when they have a class,
    /// such as `Makers`. These classes are either ambiguous or not used by the
    /// SSCs in a meaningful way.
    pub fn class(&self) -> Option<AuthorityClass> {
        match self {
            AuthorityKind::AdminAdm(_) => Some(AuthorityClass::Admins),
            AuthorityKind::AdminSec(_) => Some(AuthorityClass::Admins),
            AuthorityKind::User(_) => Some(AuthorityClass::Users),
            AuthorityKind::BandMaster(_) => Some(AuthorityClass::BandMasters),
            _ => None,
        }
    }
}

impl From<AuthorityRef> for AuthorityKind {
    fn from(value: AuthorityRef) -> Self {
        use core::shared::authority::{ANYBODY, SID};
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use opal_2::admin::authority::ADMIN as ADMIN_SP_ADMIN;
        use opal_2::locking::authority::{ADMIN as SECONDARY_SP_ADMIN, USER};
        use psid::admin::authority::PSID;

        match value {
            ANYBODY => Self::Anybody,
            _ if let Ok(class) = AuthorityClass::try_from(value) => Self::Class(class),
            SID => Self::Sid,
            PSID => Self::Psid,
            _ if let Some(index) = ADMIN_SP_ADMIN.index_of(value) => Self::AdminAdm(index),
            _ if let Some(index) = SECONDARY_SP_ADMIN.index_of(value) => Self::AdminSec(index),
            _ if let Some(index) = USER.index_of(value) => Self::User(index),
            _ if let Some(index) = BAND_MASTER.index_of(value) => Self::BandMaster(index),
            ERASE_MASTER => Self::EraseMaster,
            other => Self::Other(other),
        }
    }
}

impl From<AuthorityKind> for AuthorityRef {
    fn from(value: AuthorityKind) -> Self {
        use core::shared::authority::{ANYBODY, SID};
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use opal_2::admin::authority::ADMIN as ADMIN_SP_ADMIN;
        use opal_2::locking::authority::{ADMIN as SECONDARY_SP_ADMIN, USER};
        use psid::admin::authority::PSID;

        match value {
            AuthorityKind::Anybody => ANYBODY,
            AuthorityKind::Class(class) => class.into(),
            AuthorityKind::Sid => SID,
            AuthorityKind::Psid => PSID,
            AuthorityKind::AdminAdm(index) => ADMIN_SP_ADMIN.get(index).expect("valid index assumed"),
            AuthorityKind::AdminSec(index) => SECONDARY_SP_ADMIN.get(index).expect("valid index assumed"),
            AuthorityKind::User(index) => USER.get(index).expect("valid index assumed"),
            AuthorityKind::BandMaster(index) => BAND_MASTER.get(index).expect("valid index assumed"),
            AuthorityKind::EraseMaster => ERASE_MASTER,
            AuthorityKind::Other(authority) => authority,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuthorityClass {
    /// The `Admins` class either on the Admin SP, the Locking SP, or the KPIO SP.
    /// Applicable to all SSCs.
    Admins,
    /// The `Users` class on the Locking SP. Applicable to the Opal family of
    /// SSCs.
    Users,
    /// The `BandMasters` class on the Locking SP of the Enterprise SSC.
    BandMasters,
}

impl TryFrom<AuthorityRef> for AuthorityClass {
    type Error = ();

    fn try_from(value: AuthorityRef) -> Result<Self, Self::Error> {
        use core::shared::authority::ADMINS;
        use enterprise::locking::authority::BAND_MASTERS;
        use opal_2::locking::authority::USERS;

        match value {
            ADMINS => Ok(Self::Admins),
            USERS => Ok(Self::Users),
            BAND_MASTERS => Ok(Self::BandMasters),
            _ => Err(()),
        }
    }
}

impl From<AuthorityClass> for AuthorityRef {
    fn from(value: AuthorityClass) -> Self {
        use core::shared::authority::ADMINS;
        use enterprise::locking::authority::BAND_MASTERS;
        use opal_2::locking::authority::USERS;

        match value {
            AuthorityClass::Admins => ADMINS,
            AuthorityClass::Users => USERS,
            AuthorityClass::BandMasters => BAND_MASTERS,
        }
    }
}

/// Determines the class of the authority based on its UID.
///
/// Uses [`AuthorityKind::class`] internally.
pub fn get_authority_class(authority: AuthorityRef) -> Option<AuthorityRef> {
    AuthorityKind::from(authority).class().map(Into::into)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::uid_registry::core::shared::authority::{ADMINS, ANYBODY, MAKERS, SID};
    use crate::uid_registry::enterprise::locking::authority::{BAND_MASTER, BAND_MASTERS, ERASE_MASTER};
    use crate::uid_registry::kpio;
    use crate::uid_registry::opal_2::admin::authority::ADMIN as ADMIN_SP_ADMIN;
    use crate::uid_registry::opal_2::locking::authority::{ADMIN as SECONDARY_SP_ADMIN, USER, USERS};
    use crate::uid_registry::psid::admin::authority::PSID;

    /// The classification relies on the authorities sharing UIDs across SSCs
    /// and SPs. These UIDs should be stable, but we want to document the
    /// equalities the code relies on.
    #[test]
    fn sscs_shared_uids_for_same_concept() {
        assert_eq!(ADMINS, opal_2::admin::authority::ADMINS);
        assert_eq!(ADMINS, opal_2::locking::authority::ADMINS);
        assert_eq!(ADMINS, kpio::admin::authority::ADMINS);
        assert_eq!(ADMINS, kpio::key_per_io::authority::ADMINS);
        assert_eq!(ADMIN_SP_ADMIN, kpio::admin::authority::ADMIN);
        assert_eq!(SECONDARY_SP_ADMIN, kpio::key_per_io::authority::ADMIN);
        assert_eq!(SID, opal_2::admin::authority::SID);
        assert_eq!(SID, enterprise::admin::authority::SID);
        assert_eq!(ANYBODY, enterprise::locking::authority::ANYBODY);
    }

    #[rstest]
    #[case::anybody(ANYBODY, AuthorityKind::Anybody)]
    #[case::admins(ADMINS, AuthorityKind::Class(AuthorityClass::Admins))]
    #[case::users(USERS, AuthorityKind::Class(AuthorityClass::Users))]
    #[case::band_masters(BAND_MASTERS, AuthorityKind::Class(AuthorityClass::BandMasters))]
    #[case::sid(SID, AuthorityKind::Sid)]
    #[case::psid(PSID, AuthorityKind::Psid)]
    #[case::admin_sp_admin_first(ADMIN_SP_ADMIN.get(0).unwrap(), AuthorityKind::AdminAdm(0))]
    #[case::admin_sp_admin_last(ADMIN_SP_ADMIN.get(ADMIN_SP_ADMIN.len() - 1).unwrap(), AuthorityKind::AdminAdm(ADMIN_SP_ADMIN.len() - 1))]
    #[case::secondary_sp_admin_first(SECONDARY_SP_ADMIN.get(0).unwrap(), AuthorityKind::AdminSec(0))]
    #[case::secondary_sp_admin_last(SECONDARY_SP_ADMIN.get(SECONDARY_SP_ADMIN.len() - 1).unwrap(), AuthorityKind::AdminSec(SECONDARY_SP_ADMIN.len() - 1))]
    #[case::user_first(USER.get(0).unwrap(), AuthorityKind::User(0))]
    #[case::user_last(USER.get(USER.len() - 1).unwrap(), AuthorityKind::User(USER.len() - 1))]
    #[case::band_master_first(BAND_MASTER.get(0).unwrap(), AuthorityKind::BandMaster(0))]
    #[case::band_master_last(BAND_MASTER.get(BAND_MASTER.len() - 1).unwrap(), AuthorityKind::BandMaster(BAND_MASTER.len() - 1))]
    #[case::erase_master(ERASE_MASTER, AuthorityKind::EraseMaster)]
    #[case::other(MAKERS, AuthorityKind::Other(MAKERS))]
    fn kind_roundtrip(#[case] authority: AuthorityRef, #[case] kind: AuthorityKind) {
        assert_eq!(AuthorityKind::from(authority), kind);
        assert_eq!(AuthorityRef::from(kind), authority);
    }

    #[rstest]
    #[case::anybody(AuthorityKind::Anybody, None)]
    #[case::class(AuthorityKind::Class(AuthorityClass::Admins), None)]
    #[case::sid(AuthorityKind::Sid, None)]
    #[case::psid(AuthorityKind::Psid, None)]
    #[case::admin_sp_admin(AuthorityKind::AdminAdm(0), Some(AuthorityClass::Admins))]
    #[case::secondary_sp_admin(AuthorityKind::AdminSec(0), Some(AuthorityClass::Admins))]
    #[case::user(AuthorityKind::User(0), Some(AuthorityClass::Users))]
    #[case::band_master(AuthorityKind::BandMaster(0), Some(AuthorityClass::BandMasters))]
    #[case::erase_master(AuthorityKind::EraseMaster, None)]
    #[case::other(AuthorityKind::Other(MAKERS), None)]
    fn kind_class(#[case] kind: AuthorityKind, #[case] expected: Option<AuthorityClass>) {
        assert_eq!(kind.class(), expected);
    }

    #[rstest]
    #[case::admins(AuthorityClass::Admins, ADMINS)]
    #[case::users(AuthorityClass::Users, USERS)]
    #[case::band_masters(AuthorityClass::BandMasters, BAND_MASTERS)]
    fn class_round_trip(#[case] class: AuthorityClass, #[case] authority: AuthorityRef) {
        assert_eq!(AuthorityRef::from(class), authority);
        assert_eq!(AuthorityClass::try_from(authority), Ok(class));
    }

    #[rstest]
    #[case::anybody(ANYBODY)]
    #[case::makers(MAKERS)]
    #[case::individual(USER.get(0).unwrap())]
    fn class_try_from_non_class(#[case] authority: AuthorityRef) {
        assert_eq!(AuthorityClass::try_from(authority), Err(()));
    }

    #[rstest]
    #[case::has_class(ADMIN_SP_ADMIN.get(1).unwrap(), Some(ADMINS))]
    #[case::has_no_class(SID, None)]
    fn get_authority_class_(#[case] authority: AuthorityRef, #[case] expected: Option<AuthorityRef>) {
        assert_eq!(get_authority_class(authority), expected);
    }
}
