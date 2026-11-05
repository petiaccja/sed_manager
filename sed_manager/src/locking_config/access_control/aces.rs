//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_packet::UidRange;
use sed_spec::ace_expr;
use sed_spec::methods::MethodStatus;
use sed_spec::objects::{
    AceRef, AceRefExt, AuthorityKind, AuthorityRef, KAes128Ref, KAes128RefExt, KAes256Ref, KAes256RefExt,
    LockingRangeRef,
};
use sed_spec::preconfig::core::shared::table_id;
use sed_spec::preconfig::{core, enterprise, opal_2};
use sed_spec::types::AceOperand;
use sed_tper::{Error as TperError, Session};

use crate::Error;
use crate::locking_config::access_control::ace_expr::AceExpr;
use crate::ssc::Ssc;

/// The access control settings of the device.
///
/// This structure only contains the part that is important to configure, and is
/// not meant to be an exhaustive list.
///
/// For SSCs that do not allow setting a certain ACE, [`Fixed`] expressions which
/// reflect the preconfiguration are returned. When the ACE can not be read due
/// to a lack of permission, [`Assumed`] values are filled in, which are also
/// equal to the preconfiguration.
///
/// [`Fixed`]: AceExpr::Fixed
/// [`Assumed`]: AceExpr::Assumed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aces {
    pub global: GlobalAces,
    pub mbr: Option<MbrAces>,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangeAces>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityAces>,
}

impl Aces {
    pub async fn load(session: &Session, ssc: Ssc) -> Result<Self, Error> {
        let key_table_support = KeyTableSupport::query(session, ssc).await?;
        let locking_ranges_refs = session.next(None, None).await?;
        let mut locking_ranges = BTreeMap::new();
        for uid in locking_ranges_refs {
            locking_ranges.insert(uid, LockingRangeAces::load(session, uid, ssc, key_table_support).await?);
        }

        let authorities_refs = session.next(None, None).await?;
        let mut authorities = BTreeMap::new();
        for uid in authorities_refs {
            authorities.insert(uid, AuthorityAces::load(session, uid, ssc).await?);
        }

        Ok(Self {
            global: GlobalAces::load(session, ssc).await?,
            mbr: MbrAces::load(session, ssc).await?,
            locking_ranges,
            authorities,
        })
    }
}

/// ACE expressions that are globally important, i.e. not tied to a single
/// authority or locking range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalAces {
    pub read_ace_table: AceExpr,
    pub set_ace_expressions: AceExpr,
    pub read_authority_table: AceExpr,
    pub read_data_store: AceExpr,
    pub write_data_store: AceExpr,
}

impl GlobalAces {
    pub async fn load(session: &Session, ssc: Ssc) -> Result<Self, Error> {
        Ok(Self {
            read_ace_table: Self::read_ace_table_source(ssc).load(session).await?,
            set_ace_expressions: Self::set_ace_expressions_source(ssc).load(session).await?,
            read_authority_table: Self::read_authority_table_source(ssc).load(session).await?,
            read_data_store: Self::read_data_store_source(ssc).load(session).await?,
            write_data_store: Self::write_data_store_source(ssc).load(session).await?,
        })
    }

    fn read_ace_table_source(ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::ACE_GET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!()),
            _ => AceSource::Editable { ace: ACE_GET_ALL, default: ace_expr!(ADMINS) },
        }
    }

    fn set_ace_expressions_source(ssc: Ssc) -> AceSource {
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!()),
            _ => AceSource::Fixed(ace_expr!(ADMINS)),
        }
    }

    fn read_authority_table_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::ace::AUTHORITY_GET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            // `AnyMaster` is needed to call `Next` on the authority table. The
            // authority objects themselves are only readable by themselves.
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)),
            _ => AceSource::Editable { ace: AUTHORITY_GET_ALL, default: ace_expr!(ADMINS) },
        }
    }

    fn read_data_store_source(ssc: Ssc) -> AceSource {
        use core::shared::authority::ANYBODY;
        use opal_2::locking::ace::DATA_STORE_GET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(ANYBODY)),
            _ => AceSource::Editable { ace: DATA_STORE_GET_ALL, default: ace_expr!(ADMINS) },
        }
    }

    fn write_data_store_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::BAND_MASTERS;
        use opal_2::locking::ace::DATA_STORE_SET_ALL;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS)),
            _ => AceSource::Editable { ace: DATA_STORE_SET_ALL, default: ace_expr!(ADMINS) },
        }
    }
}

/// ACE expressions that are control access to the `MBR` and `MBRControl` tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbrAces {
    pub set_enable_to_dor: AceExpr,
    pub set_done: AceExpr,
    pub upload: AceExpr,
}

impl MbrAces {
    /// Load the MBR ACEs, or [`None`] if the device does not support shadow
    /// MBR.
    pub async fn load(session: &Session, ssc: Ssc) -> Result<Option<Self>, Error> {
        // The MBR is optional for some SSCs (e.g. Pyrite, Enterprise). The MBR
        // table is readable by `Anybody`, so this works regardless of the authority.
        // A missing table may be reported as an invalid object, or as a missing
        // row in the AccessControl table.
        match session.get_bytes(table_id::MBR, 0..1).await {
            Ok(_) => (),
            Err(TperError::MethodCallFailed(MethodStatus::InvalidParameter | MethodStatus::NotAuthorized)) => {
                return Ok(None);
            }
            Err(err) => return Err(err.into()),
        }
        Ok(Some(Self {
            set_enable_to_dor: Self::set_enable_to_dor_source(ssc).load(session).await?,
            set_done: Self::set_done_source(ssc).load(session).await?,
            upload: Self::upload_source(ssc).load(session).await?,
        }))
    }

    fn set_enable_to_dor_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::KeyPerIO => AceSource::Fixed(ace_expr!()),
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)),
            _ => AceSource::Fixed(ace_expr!(ADMINS)),
        }
    }

    fn set_done_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::ace::MBR_CONTROL_SET_DONE_TO_DOR;
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::KeyPerIO => AceSource::Fixed(ace_expr!()),
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)),
            // Opal 1.0 calls it `ACE_MBRControl_Set_Done`, and it covers only
            // `Done`, but it has the same UID.
            _ => AceSource::Editable { ace: MBR_CONTROL_SET_DONE_TO_DOR, default: ace_expr!(ADMINS) },
        }
    }

    fn upload_source(ssc: Ssc) -> AceSource {
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::authority::ADMINS;

        match ssc {
            Ssc::KeyPerIO => AceSource::Fixed(ace_expr!()),
            Ssc::Enterprise => AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)),
            // `ACE_Admin`, which is not editable.
            _ => AceSource::Fixed(ace_expr!(ADMINS)),
        }
    }
}

/// ACE expressions that control access to a single locking range object.
///
/// Both the 128-bit and 256-bit key ACEs are tracked for devices that support
/// both. Authorities that can't read the `ActiveKey` of the range can still
/// work out permissions if both ACEs are known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangeAces {
    pub read_start_to_active_key: AceExpr,
    pub set_read_locked: AceExpr,
    pub set_write_locked: AceExpr,
    pub erase_128: Option<AceExpr>,
    pub erase_256: Option<AceExpr>,
    pub set_start_to_lor: AceExpr,
}

impl LockingRangeAces {
    pub async fn load(
        session: &Session,
        uid: LockingRangeRef,
        ssc: Ssc,
        key_table_support: KeyTableSupport,
    ) -> Result<Self, Error> {
        let erase_128 = match key_table_support.aes_128 {
            true => Some(Self::erase_128_source(uid, ssc).load(session).await?),
            false => None,
        };
        let erase_256 = match key_table_support.aes_256 {
            true => Some(Self::erase_256_source(uid, ssc).load(session).await?),
            false => None,
        };
        Ok(Self {
            read_start_to_active_key: Self::read_start_to_active_key_source(uid, ssc).load(session).await?,
            set_read_locked: Self::set_read_locked_source(uid, ssc).load(session).await?,
            set_write_locked: Self::set_write_locked_source(uid, ssc).load(session).await?,
            erase_128,
            erase_256,
            set_start_to_lor: Self::set_start_to_lor_source(uid, ssc).load(session).await?,
        })
    }

    fn read_start_to_active_key_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use Ssc::*;
        use core::shared::authority::ANYBODY;
        use opal_2::locking::ace::{
            LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY,
        };
        use opal_2::locking::authority::ADMINS;
        use opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

        match ssc {
            // `Anybody_GetBand` covers `UID` to `ActiveKey`.
            Enterprise if enterprise_range_owner(uid).is_some() => AceSource::Fixed(ace_expr!(ANYBODY)),
            Enterprise | KeyPerIO => AceSource::Fixed(ace_expr!()),
            _ if uid == GLOBAL_RANGE => AceSource::Editable {
                ace: LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY,
                default: ace_expr!(ADMINS),
            },
            _ if let Some(ace) =
                RANGE.index_of(uid).and_then(|index| LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY.get(index)) =>
            {
                AceSource::Editable { ace, default: ace_expr!(ADMINS) }
            }
            _ => AceSource::Fixed(ace_expr!()),
        }
    }

    fn set_read_locked_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::{LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, LOCKING_RANGE_SET_RD_LOCKED};
        set_locked_source(uid, ssc, LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, |index| LOCKING_RANGE_SET_RD_LOCKED.get(index))
    }

    fn set_write_locked_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::{LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, LOCKING_RANGE_SET_WR_LOCKED};
        set_locked_source(uid, ssc, LOCKING_GLOBAL_RANGE_SET_WR_LOCKED, |index| LOCKING_RANGE_SET_WR_LOCKED.get(index))
    }

    fn set_start_to_lor_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use Ssc::*;
        use opal_2::locking::authority::ADMINS;
        use opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

        match ssc {
            // `BandMaster{n}_SetBand` (`ReadLockEnabled` to `LockOnReset` for
            // the global range, `RangeStart` to `LockOnReset` for the others).
            Enterprise if let Some(owner) = enterprise_range_owner(uid) => AceSource::Fixed(ace_expr!(owner)),
            Enterprise | KeyPerIO => AceSource::Fixed(ace_expr!()),
            // `ACE_Locking_GlblRng_Admins_Set` and `ACE_Locking_Admins_RangeStartToLOR`.
            _ if uid == GLOBAL_RANGE || RANGE.contains(uid) => AceSource::Fixed(ace_expr!(ADMINS)),
            _ => AceSource::Fixed(ace_expr!()),
        }
    }

    fn erase_128_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::{K_AES_128_GLOBAL_RANGE_GEN_KEY, K_AES_128_RANGE_GEN_KEY};
        erase_source(uid, ssc, K_AES_128_GLOBAL_RANGE_GEN_KEY, K_AES_128_RANGE_GEN_KEY)
    }

    fn erase_256_source(uid: LockingRangeRef, ssc: Ssc) -> AceSource {
        use opal_2::locking::ace::{K_AES_256_GLOBAL_RANGE_GEN_KEY, K_AES_256_RANGE_GEN_KEY};
        erase_source(uid, ssc, K_AES_256_GLOBAL_RANGE_GEN_KEY, K_AES_256_RANGE_GEN_KEY)
    }
}

/// Get the ACE source for erasing a locking range.
fn erase_source(uid: LockingRangeRef, ssc: Ssc, global_range_ace: AceRef, range_ace: UidRange<AceRef>) -> AceSource {
    use Ssc::*;
    use enterprise::locking::authority::ERASE_MASTER;
    use opal_2::locking::authority::ADMINS;
    use opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

    match ssc {
        Enterprise if enterprise_range_owner(uid).is_some() => AceSource::Fixed(ace_expr!(ERASE_MASTER)),
        // Pyrite has no media encryption, and KPIO has no locking ranges.
        Enterprise | PyriteV1 | PyriteV2 | KeyPerIO => AceSource::Fixed(ace_expr!()),
        _ if uid == GLOBAL_RANGE => AceSource::Editable { ace: global_range_ace, default: ace_expr!(ADMINS) },
        _ if let Some(ace) = RANGE.index_of(uid).and_then(|index| range_ace.get(index)) => {
            AceSource::Editable { ace, default: ace_expr!(ADMINS) }
        }
        _ => AceSource::Fixed(ace_expr!()),
    }
}

/// The `Set_RdLocked` and `Set_WrLocked` ACEs of a locking range follow the
/// same pattern.
fn set_locked_source(
    uid: LockingRangeRef,
    ssc: Ssc,
    global_range_ace: AceRef,
    range_ace: impl Fn(usize) -> Option<AceRef>,
) -> AceSource {
    use Ssc::*;
    use opal_2::locking::authority::ADMINS;
    use opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

    match ssc {
        // `BandMaster{n}_SetBand` covers both `ReadLocked` and `WriteLocked`.
        Enterprise if let Some(owner) = enterprise_range_owner(uid) => AceSource::Fixed(ace_expr!(owner)),
        Enterprise | KeyPerIO => AceSource::Fixed(ace_expr!()),
        _ if uid == GLOBAL_RANGE => AceSource::Editable { ace: global_range_ace, default: ace_expr!(ADMINS) },
        _ if let Some(ace) = RANGE.index_of(uid).and_then(range_ace) => {
            AceSource::Editable { ace, default: ace_expr!(ADMINS) }
        }
        _ => AceSource::Fixed(ace_expr!()),
    }
}

/// The `BandMaster` authority that owns an Enterprise locking range.
fn enterprise_range_owner(uid: LockingRangeRef) -> Option<AuthorityRef> {
    use enterprise::locking::authority::BAND_MASTER;
    use enterprise::locking::locking::{BAND, GLOBAL_RANGE};

    if uid == GLOBAL_RANGE {
        BAND_MASTER.get(0)
    } else {
        BAND_MASTER.get(BAND.index_of(uid)? + 1)
    }
}

/// Which media encryption key tables the device has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyTableSupport {
    pub aes_128: bool,
    pub aes_256: bool,
}

impl KeyTableSupport {
    /// Determine which of the `K_AES_128` and `K_AES_256` tables the device
    /// supports.
    pub async fn query(session: &Session, ssc: Ssc) -> Result<Self, Error> {
        // The `GlobalRange` exists on all devices, and the `K_AES_*` tables'
        // `Mode` column is readable by `Anybody`, so that's a good probe.
        let (key_128, key_256) = Self::global_range_keys(ssc);
        Ok(Self {
            aes_128: Self::evaluate_probe_result(session.get_field(key_128.mode()).await)?,
            aes_256: Self::evaluate_probe_result(session.get_field(key_256.mode()).await)?,
        })
    }

    fn global_range_keys(ssc: Ssc) -> (KAes128Ref, KAes256Ref) {
        match ssc {
            Ssc::Enterprise => {
                use enterprise::locking::{k_aes_128, k_aes_256};
                (k_aes_128::GLOBAL_RANGE_KEY, k_aes_256::GLOBAL_RANGE_KEY)
            }
            _ => {
                use opal_2::locking::{k_aes_128, k_aes_256};
                (k_aes_128::GLOBAL_RANGE_KEY, k_aes_256::GLOBAL_RANGE_KEY)
            }
        }
    }

    /// Interpret the result of reading the probed key's `Mode`.
    fn evaluate_probe_result<T>(result: Result<T, TperError>) -> Result<bool, Error> {
        match result {
            Ok(_) => Ok(true),
            // The object exists, but the column was not returned.
            Err(TperError::FieldNotReturned { .. }) => Ok(true),
            // A missing table may be reported as an invalid object, or as a
            // missing row in the AccessControl table.
            Err(TperError::MethodCallFailed(MethodStatus::InvalidParameter | MethodStatus::NotAuthorized)) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}

/// ACE expressions that control access to a single authority object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityAces {
    pub set_enabled: AceExpr,
    pub set_common_name: AceExpr,
    pub set_pin: AceExpr,
}

impl AuthorityAces {
    pub async fn load(session: &Session, uid: AuthorityRef, ssc: Ssc) -> Result<Self, Error> {
        let set_enabled = Self::set_enabled_source(uid, ssc).load(session).await?;
        let set_common_name = Self::set_common_name_source(uid, ssc).load(session).await?;
        let set_pin = Self::set_pin_source(uid, ssc).load(session).await?;
        Ok(Self { set_enabled, set_common_name, set_pin })
    }

    fn set_enabled_source(uid: AuthorityRef, ssc: Ssc) -> AceSource {
        use AuthorityKind::{AdminSec, User};
        use opal_2::locking::authority::ADMINS;

        let kind = AuthorityKind::from(uid);

        match (ssc, kind) {
            (_, AdminSec(0)) => AceSource::Fixed(ace_expr!()),
            (_, AdminSec(_) | User(_)) => AceSource::Fixed(ace_expr!(ADMINS)),
            _ => AceSource::Fixed(ace_expr!()),
        }
    }

    fn set_common_name_source(uid: AuthorityRef, ssc: Ssc) -> AceSource {
        use AuthorityKind::{AdminSec, User};
        use Ssc::*;
        use opal_2::locking::ace::USER_SET_COMMON_NAME;
        use opal_2::locking::authority::ADMINS;

        let kind = AuthorityKind::from(uid);

        match (ssc, kind) {
            (OpalV2 | Ruby | KeyPerIO, AdminSec(_)) => AceSource::Fixed(ace_expr!(ADMINS)),
            (OpalV1 | Opalite | PyriteV1 | PyriteV2, AdminSec(_)) => AceSource::Fixed(ace_expr!()),
            (OpalV2 | Ruby, User(index)) if let Some(ace) = USER_SET_COMMON_NAME.get(index) => {
                AceSource::Editable { ace, default: ace_expr!(ADMINS) }
            }
            _ => AceSource::Fixed(ace_expr!()),
        }
    }

    fn set_pin_source(uid: AuthorityRef, ssc: Ssc) -> AceSource {
        use AuthorityKind::{AdminSec, BandMaster, EraseMaster, User};
        use Ssc::*;
        use opal_2::locking::ace::C_PIN_USER_SET_PIN;
        use opal_2::locking::authority::ADMINS;

        let kind = AuthorityKind::from(uid);

        match (ssc, kind) {
            (Enterprise, BandMaster(_) | EraseMaster) => AceSource::Fixed(ace_expr!(uid)),
            (_, AdminSec(_)) => AceSource::Fixed(ace_expr!(ADMINS)),
            (_, User(index)) if let Some(ace) = C_PIN_USER_SET_PIN.get(index) => {
                AceSource::Editable { ace, default: ace_expr!(ADMINS uid ||) }
            }
            _ => AceSource::Fixed(ace_expr!()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AceSource {
    Fixed(Vec<AceOperand>),
    Editable { ace: AceRef, default: Vec<AceOperand> },
}

impl AceSource {
    pub async fn load(self, session: &Session) -> Result<AceExpr, Error> {
        match self {
            AceSource::Fixed(expr) => Ok(AceExpr::Fixed(expr)),
            AceSource::Editable { ace, default } => match session.get_field(ace.boolean_expr()).await {
                Ok(expr) => Ok(AceExpr::Loaded(expr)),
                Err(TperError::MethodCallFailed(MethodStatus::NotAuthorized)) => Ok(AceExpr::Assumed(default)),
                Err(TperError::FieldNotReturned { .. }) => Ok(AceExpr::Assumed(default)),
                Err(err) => Err(err.into()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Ssc::*;

    use rstest::rstest;
    use sed_spec::preconfig::core as core_preconfig;

    const OPAL2_RUBY: [Ssc; 2] = [OpalV2, Ruby];
    const O1_OL_P1_P2: [Ssc; 4] = [OpalV1, Opalite, PyriteV1, PyriteV2];
    const OPAL_FAMILY: [Ssc; 6] = [OpalV1, OpalV2, Opalite, PyriteV1, PyriteV2, Ruby];
    const OPAL_FAMILY_KPIO: [Ssc; 7] = [OpalV1, OpalV2, Opalite, PyriteV1, PyriteV2, Ruby, KeyPerIO];

    //--------------------------------------------------------------------------
    // Authority aces
    //--------------------------------------------------------------------------

    mod authority_aces {
        use super::*;

        use core_preconfig::shared::authority::ANYBODY;
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use opal_2::locking::ace::{C_PIN_USER_SET_PIN, USER_SET_COMMON_NAME};
        use opal_2::locking::authority::{ADMIN, ADMINS, USER};

        #[rstest]
        #[case::opal_anybody(&OPAL_FAMILY, ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::opal_admins(&OPAL_FAMILY, ADMINS, AceSource::Fixed(ace_expr!()))]
        #[case::opal_admin1(&OPAL_FAMILY, ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_admin2(&OPAL_FAMILY, ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_user1(&OPAL_FAMILY, USER.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_user2(&OPAL_FAMILY, USER.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::kpio_admin1(&[KeyPerIO], ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio_admin2(&[KeyPerIO], ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise_anybody(&[Enterprise], ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_em(&[Enterprise], ERASE_MASTER, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_bm0(&[Enterprise], BAND_MASTER.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_bm1(&[Enterprise], BAND_MASTER.get(1).unwrap(), AceSource::Fixed(ace_expr!()))]
        fn set_enabled_source(#[case] sscs: &[Ssc], #[case] authority: AuthorityRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(AuthorityAces::set_enabled_source(authority, *ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal_anybody(&OPAL2_RUBY, ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::opal_admin1(&OPAL2_RUBY, ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_admin2(&OPAL2_RUBY, ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_user1(&OPAL2_RUBY, USER.get(0).unwrap(), AceSource::Editable{ ace: USER_SET_COMMON_NAME.get(0).unwrap(), default: ace_expr!(ADMINS) })]
        #[case::opal_user2(&OPAL2_RUBY, USER.get(1).unwrap(), AceSource::Editable{ ace: USER_SET_COMMON_NAME.get(1).unwrap(), default: ace_expr!(ADMINS) })]
        #[case::pyrite_anybody(&O1_OL_P1_P2, ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::pyrite_admin1(&O1_OL_P1_P2, ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::pyrite_admin2(&O1_OL_P1_P2, ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::pyrite_user1(&O1_OL_P1_P2, USER.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::pyrite_user2(&O1_OL_P1_P2, USER.get(1).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio_admin1(&[KeyPerIO], ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::kpio_admin2(&[KeyPerIO], ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise_anybody(&[Enterprise], ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_em(&[Enterprise], ERASE_MASTER, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_bm0(&[Enterprise], BAND_MASTER.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_bm1(&[Enterprise], BAND_MASTER.get(1).unwrap(), AceSource::Fixed(ace_expr!()))]
        fn set_common_name_source(#[case] sscs: &[Ssc], #[case] authority: AuthorityRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(AuthorityAces::set_common_name_source(authority, *ssc), source);
            }
        }

        #[rstest]
        #[case::opal_anybody(&OPAL_FAMILY, ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::opal_admins(&OPAL_FAMILY, ADMINS, AceSource::Fixed(ace_expr!()))]
        #[case::opal_admin1(&OPAL_FAMILY, ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_admin2(&OPAL_FAMILY, ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_user1(&OPAL_FAMILY, USER.get(0).unwrap(), AceSource::Editable{ ace: C_PIN_USER_SET_PIN.get(0).unwrap(), default: ace_expr!(ADMINS (USER.get(0).unwrap()) ||) })]
        #[case::opal_user2(&OPAL_FAMILY, USER.get(1).unwrap(), AceSource::Editable{ ace: C_PIN_USER_SET_PIN.get(1).unwrap(), default: ace_expr!(ADMINS (USER.get(1).unwrap()) ||) })]
        #[case::opal_user_without_ace(&OPAL_FAMILY, USER.get(C_PIN_USER_SET_PIN.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio_admin1(&[KeyPerIO], ADMIN.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::kpio_admin2(&[KeyPerIO], ADMIN.get(1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise_anybody(&[Enterprise], ANYBODY, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_em(&[Enterprise], ERASE_MASTER, AceSource::Fixed(ace_expr!(ERASE_MASTER)))]
        #[case::enterprise_bm0(&[Enterprise], BAND_MASTER.get(0).unwrap(), AceSource::Fixed(ace_expr!((BAND_MASTER.get(0).unwrap()))))]
        #[case::enterprise_bm1(&[Enterprise], BAND_MASTER.get(1).unwrap(), AceSource::Fixed(ace_expr!((BAND_MASTER.get(1).unwrap()))))]
        fn set_pin_source(#[case] sscs: &[Ssc], #[case] authority: AuthorityRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(AuthorityAces::set_pin_source(authority, *ssc), source, "{ssc:?}");
            }
        }
    }

    //--------------------------------------------------------------------------
    // Global aces
    //--------------------------------------------------------------------------

    mod global_aces {
        use super::*;

        use core_preconfig::shared::authority::ANYBODY;
        use enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};
        use opal_2::locking::ace::{ACE_GET_ALL, AUTHORITY_GET_ALL, DATA_STORE_GET_ALL, DATA_STORE_SET_ALL};
        use opal_2::locking::authority::ADMINS;

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, AceSource::Editable { ace: ACE_GET_ALL, default: ace_expr!(ADMINS) })]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!()))]
        fn read_ace_table_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(GlobalAces::read_ace_table_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!()))]
        fn set_ace_expressions_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(GlobalAces::set_ace_expressions_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, AceSource::Editable { ace: AUTHORITY_GET_ALL, default: ace_expr!(ADMINS) })]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)))]
        fn read_authority_table_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(GlobalAces::read_authority_table_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, AceSource::Editable { ace: DATA_STORE_GET_ALL, default: ace_expr!(ADMINS) })]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(ANYBODY)))]
        fn read_data_store_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(GlobalAces::read_data_store_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, AceSource::Editable { ace: DATA_STORE_SET_ALL, default: ace_expr!(ADMINS) })]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(BAND_MASTERS)))]
        fn write_data_store_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(GlobalAces::write_data_store_source(*ssc), source, "{ssc:?}");
            }
        }
    }

    //--------------------------------------------------------------------------
    // MBR aces
    //--------------------------------------------------------------------------

    mod mbr_aces {
        use super::*;

        use opal_2::locking::ace::MBR_CONTROL_SET_DONE_TO_DOR;
        use opal_2::locking::authority::ADMINS;
        use sed_spec::preconfig::enterprise::locking::authority::{BAND_MASTERS, ERASE_MASTER};

        #[rstest]
        #[case::opal(&OPAL_FAMILY, AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)))]
        #[case::unsupported(&[KeyPerIO], AceSource::Fixed(ace_expr!()))]
        fn set_enable_to_dor_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(MbrAces::set_enable_to_dor_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY, AceSource::Editable { ace: MBR_CONTROL_SET_DONE_TO_DOR, default: ace_expr!(ADMINS) })]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)))]
        #[case::unsupported(&[KeyPerIO], AceSource::Fixed(ace_expr!()))]
        fn set_done_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(MbrAces::set_done_source(*ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal(&OPAL_FAMILY, AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::enterprise(&[Enterprise], AceSource::Fixed(ace_expr!(BAND_MASTERS ERASE_MASTER ||)))]
        #[case::unsupported(&[KeyPerIO], AceSource::Fixed(ace_expr!()))]
        fn upload_source(#[case] sscs: &[Ssc], #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(MbrAces::upload_source(*ssc), source, "{ssc:?}");
            }
        }
    }

    //--------------------------------------------------------------------------
    // Locking range aces
    //--------------------------------------------------------------------------

    mod locking_range_aces {
        use super::*;

        use core_preconfig::shared::authority::ANYBODY;
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use enterprise::locking::locking::BAND;
        use opal_2::locking::ace::{
            K_AES_128_GLOBAL_RANGE_GEN_KEY, K_AES_128_RANGE_GEN_KEY, K_AES_256_GLOBAL_RANGE_GEN_KEY,
            K_AES_256_RANGE_GEN_KEY, LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY,
            LOCKING_GLOBAL_RANGE_SET_RD_LOCKED, LOCKING_GLOBAL_RANGE_SET_WR_LOCKED,
            LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY, LOCKING_RANGE_SET_RD_LOCKED, LOCKING_RANGE_SET_WR_LOCKED,
        };
        use opal_2::locking::authority::ADMINS;
        use opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

        fn editable(ace: AceRef) -> AceSource {
            AceSource::Editable { ace, default: ace_expr!(ADMINS) }
        }

        fn fixed_band_master(index: usize) -> AceSource {
            AceSource::Fixed(ace_expr!((BAND_MASTER.get(index).unwrap())))
        }

        #[test]
        fn enterprise_and_opal_share_the_global_range() {
            assert_eq!(GLOBAL_RANGE, enterprise::locking::locking::GLOBAL_RANGE);
        }

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY, GLOBAL_RANGE, editable(LOCKING_GLOBAL_RANGE_GET_RANGE_START_TO_ACTIVE_KEY))]
        #[case::opal_range1(&OPAL_FAMILY, RANGE.get(0).unwrap(), editable(LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY.get(0).unwrap()))]
        #[case::opal_range2(&OPAL_FAMILY, RANGE.get(1).unwrap(), editable(LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY.get(1).unwrap()))]
        #[case::opal_range_without_ace(&OPAL_FAMILY, RANGE.get(LOCKING_RANGE_GET_RANGE_START_TO_ACTIVE_KEY.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_enterprise_band(&OPAL_FAMILY, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, AceSource::Fixed(ace_expr!(ANYBODY)))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!(ANYBODY)))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn read_start_to_active_key_source(
            #[case] sscs: &[Ssc],
            #[case] range: LockingRangeRef,
            #[case] source: AceSource,
        ) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::read_start_to_active_key_source(range, *ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY, GLOBAL_RANGE, editable(LOCKING_GLOBAL_RANGE_SET_RD_LOCKED))]
        #[case::opal_range1(&OPAL_FAMILY, RANGE.get(0).unwrap(), editable(LOCKING_RANGE_SET_RD_LOCKED.get(0).unwrap()))]
        #[case::opal_range2(&OPAL_FAMILY, RANGE.get(1).unwrap(), editable(LOCKING_RANGE_SET_RD_LOCKED.get(1).unwrap()))]
        #[case::opal_range_without_ace(&OPAL_FAMILY, RANGE.get(LOCKING_RANGE_SET_RD_LOCKED.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_enterprise_band(&OPAL_FAMILY, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, fixed_band_master(0))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), fixed_band_master(1))]
        #[case::enterprise_band2(&[Enterprise], BAND.get(1).unwrap(), fixed_band_master(2))]
        #[case::enterprise_band_without_band_master(&[Enterprise], BAND.get(BAND_MASTER.len() - 1).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn set_read_locked_source(#[case] sscs: &[Ssc], #[case] range: LockingRangeRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::set_read_locked_source(range, *ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY, GLOBAL_RANGE, editable(LOCKING_GLOBAL_RANGE_SET_WR_LOCKED))]
        #[case::opal_range1(&OPAL_FAMILY, RANGE.get(0).unwrap(), editable(LOCKING_RANGE_SET_WR_LOCKED.get(0).unwrap()))]
        #[case::opal_range2(&OPAL_FAMILY, RANGE.get(1).unwrap(), editable(LOCKING_RANGE_SET_WR_LOCKED.get(1).unwrap()))]
        #[case::opal_range_without_ace(&OPAL_FAMILY, RANGE.get(LOCKING_RANGE_SET_WR_LOCKED.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_enterprise_band(&OPAL_FAMILY, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, fixed_band_master(0))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), fixed_band_master(1))]
        #[case::enterprise_band2(&[Enterprise], BAND.get(1).unwrap(), fixed_band_master(2))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn set_write_locked_source(#[case] sscs: &[Ssc], #[case] range: LockingRangeRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::set_write_locked_source(range, *ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY, GLOBAL_RANGE, AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_range1(&OPAL_FAMILY, RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_last_range(&OPAL_FAMILY, RANGE.get(RANGE.len() - 1).unwrap(), AceSource::Fixed(ace_expr!(ADMINS)))]
        #[case::opal_enterprise_band(&OPAL_FAMILY, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, fixed_band_master(0))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), fixed_band_master(1))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn set_start_to_lor_source(#[case] sscs: &[Ssc], #[case] range: LockingRangeRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::set_start_to_lor_source(range, *ssc), source, "{ssc:?}");
            }
        }

        const OPAL_FAMILY_ENCRYPTED: [Ssc; 4] = [OpalV1, OpalV2, Opalite, Ruby];

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY_ENCRYPTED, GLOBAL_RANGE, editable(K_AES_128_GLOBAL_RANGE_GEN_KEY))]
        #[case::opal_range1(&OPAL_FAMILY_ENCRYPTED, RANGE.get(0).unwrap(), editable(K_AES_128_RANGE_GEN_KEY.get(0).unwrap()))]
        #[case::opal_range2(&OPAL_FAMILY_ENCRYPTED, RANGE.get(1).unwrap(), editable(K_AES_128_RANGE_GEN_KEY.get(1).unwrap()))]
        #[case::opal_range_without_ace(&OPAL_FAMILY_ENCRYPTED, RANGE.get(K_AES_128_RANGE_GEN_KEY.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_enterprise_band(&OPAL_FAMILY_ENCRYPTED, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::pyrite(&[PyriteV1, PyriteV2], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, AceSource::Fixed(ace_expr!(ERASE_MASTER)))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!(ERASE_MASTER)))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn erase_128_source(#[case] sscs: &[Ssc], #[case] range: LockingRangeRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::erase_128_source(range, *ssc), source, "{ssc:?}");
            }
        }

        #[rstest]
        #[case::opal_global(&OPAL_FAMILY_ENCRYPTED, GLOBAL_RANGE, editable(K_AES_256_GLOBAL_RANGE_GEN_KEY))]
        #[case::opal_range1(&OPAL_FAMILY_ENCRYPTED, RANGE.get(0).unwrap(), editable(K_AES_256_RANGE_GEN_KEY.get(0).unwrap()))]
        #[case::opal_range2(&OPAL_FAMILY_ENCRYPTED, RANGE.get(1).unwrap(), editable(K_AES_256_RANGE_GEN_KEY.get(1).unwrap()))]
        #[case::opal_range_without_ace(&OPAL_FAMILY_ENCRYPTED, RANGE.get(K_AES_256_RANGE_GEN_KEY.len()).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::opal_enterprise_band(&OPAL_FAMILY_ENCRYPTED, BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::pyrite(&[PyriteV1, PyriteV2], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        #[case::enterprise_global(&[Enterprise], GLOBAL_RANGE, AceSource::Fixed(ace_expr!(ERASE_MASTER)))]
        #[case::enterprise_band1(&[Enterprise], BAND.get(0).unwrap(), AceSource::Fixed(ace_expr!(ERASE_MASTER)))]
        #[case::enterprise_opal_range(&[Enterprise], RANGE.get(0).unwrap(), AceSource::Fixed(ace_expr!()))]
        #[case::kpio(&[KeyPerIO], GLOBAL_RANGE, AceSource::Fixed(ace_expr!()))]
        fn erase_256_source(#[case] sscs: &[Ssc], #[case] range: LockingRangeRef, #[case] source: AceSource) {
            for ssc in sscs {
                assert_eq!(LockingRangeAces::erase_256_source(range, *ssc), source, "{ssc:?}");
            }
        }
    }

    //--------------------------------------------------------------------------
    // Key table support
    //--------------------------------------------------------------------------

    mod key_table_support {
        use super::*;

        use sed_packet::Uid;

        #[rstest]
        #[case::opal(&OPAL_FAMILY_KPIO, opal_2::locking::k_aes_128::GLOBAL_RANGE_KEY, opal_2::locking::k_aes_256::GLOBAL_RANGE_KEY)]
        #[case::enterprise(&[Enterprise], enterprise::locking::k_aes_128::GLOBAL_RANGE_KEY, enterprise::locking::k_aes_256::GLOBAL_RANGE_KEY)]
        fn global_range_keys(#[case] sscs: &[Ssc], #[case] key_128: KAes128Ref, #[case] key_256: KAes256Ref) {
            for ssc in sscs {
                assert_eq!(KeyTableSupport::global_range_keys(*ssc), (key_128, key_256), "{ssc:?}");
            }
        }

        #[rstest]
        #[case::ok(Ok(()), Some(true))]
        #[case::field_not_returned(Err(TperError::FieldNotReturned { object: Uid::null(), field: 4 }), Some(true))]
        #[case::invalid_parameter(Err(TperError::MethodCallFailed(MethodStatus::InvalidParameter)), Some(false))]
        #[case::not_authorized(Err(TperError::MethodCallFailed(MethodStatus::NotAuthorized)), Some(false))]
        #[case::other_status(Err(TperError::MethodCallFailed(MethodStatus::Fail)), None)]
        #[case::other_error(Err(TperError::Closed), None)]
        fn exists(#[case] result: Result<(), TperError>, #[case] expected: Option<bool>) {
            assert_eq!(KeyTableSupport::evaluate_probe_result(result).ok(), expected);
        }
    }
}
