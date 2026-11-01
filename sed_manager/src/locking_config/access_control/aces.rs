//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::ace_expr;
use sed_spec::methods::MethodStatus;
use sed_spec::objects::{AceRef, AceRefExt, AuthorityRef, LockingRangeRef};
use sed_spec::preconfig::{enterprise, opal_2};
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
        let locking_ranges_refs = session.next(None, None).await?;
        let mut locking_ranges = BTreeMap::new();
        for uid in locking_ranges_refs {
            locking_ranges.insert(uid, LockingRangeAces::load(session, uid).await?);
        }

        let authorities_refs = session.next(None, None).await?;
        let mut authorities = BTreeMap::new();
        for uid in authorities_refs {
            authorities.insert(uid, AuthorityAces::load(session, uid, ssc).await?);
        }

        Ok(Self {
            global: GlobalAces::load(session).await?,
            mbr: Some(MbrAces::load(session).await?),
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
    pub async fn load(session: &Session) -> Result<Self, Error> {
        Ok(Self {
            read_ace_table: todo!(),
            set_ace_expressions: todo!(),
            read_authority_table: todo!(),
            read_data_store: todo!(),
            write_data_store: todo!(),
        })
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
    pub async fn load(session: &Session) -> Result<Self, Error> {
        Ok(Self { set_enable_to_dor: todo!(), set_done: todo!(), upload: todo!() })
    }
}

/// ACE expressions that control access to a single locking range object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangeAces {
    pub read_start_to_active_key: AceExpr,
    pub set_read_locked: AceExpr,
    pub set_write_locked: AceExpr,
    pub erase: Option<AceExpr>,
    pub set_start_to_lor: AceExpr,
}

impl LockingRangeAces {
    pub async fn load(session: &Session, uid: LockingRangeRef) -> Result<Self, Error> {
        Ok(Self {
            read_start_to_active_key: todo!(),
            set_read_locked: todo!(),
            set_write_locked: todo!(),
            erase: todo!(),
            set_start_to_lor: todo!(),
        })
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
        let set_enabled = Self::set_enabled_default(uid, ssc);
        let set_common_name =
            load_ace_or_else(session, Self::set_common_name_ref(uid, ssc), || Self::set_common_name_default(uid, ssc))
                .await?;
        let set_pin =
            load_ace_or_else(session, Self::set_pin_ref(uid, ssc), || Self::set_pin_default(uid, ssc)).await?;
        Ok(Self { set_enabled, set_common_name, set_pin })
    }

    fn set_common_name_ref(uid: AuthorityRef, ssc: Ssc) -> Option<AceRef> {
        use Ssc::*;
        use opal_2::locking::ace::{ADMINS_SET_COMMON_NAME, USER_SET_COMMON_NAME};
        use opal_2::locking::authority::{ADMIN, USER};

        match ssc {
            OpalV2 | Ruby => {
                if ADMIN.contains(uid) {
                    Some(ADMINS_SET_COMMON_NAME)
                } else if let Some(index) = USER.index_of(uid) {
                    USER_SET_COMMON_NAME.get(index)
                } else {
                    None
                }
            }
            OpalV1 | Opalite | PyriteV1 | PyriteV2 => {
                if ADMIN.contains(uid) {
                    None
                } else if let Some(index) = USER.index_of(uid) {
                    USER_SET_COMMON_NAME.get(index)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn set_pin_ref(uid: AuthorityRef, ssc: Ssc) -> Option<AceRef> {
        use Ssc::*;
        use opal_2::locking::ace::{C_PIN_ADMINS_SET_PIN, C_PIN_USER_SET_PIN};
        use opal_2::locking::authority::{ADMIN, USER};

        match ssc {
            Enterprise => None,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby => {
                if ADMIN.contains(uid) {
                    Some(C_PIN_ADMINS_SET_PIN)
                } else if let Some(index) = USER.index_of(uid) {
                    C_PIN_USER_SET_PIN.get(index)
                } else {
                    None
                }
            }
            KeyPerIO => None,
        }
    }

    fn set_enabled_default(uid: AuthorityRef, ssc: Ssc) -> AceExpr {
        use Ssc::*;
        use opal_2::locking::authority::{ADMIN, ADMINS};

        match ssc {
            OpalV2 | Ruby | OpalV1 | Opalite | PyriteV1 | PyriteV2 => {
                if uid == ADMIN.get(0).unwrap() {
                    AceExpr::Fixed(ace_expr!(ADMINS))
                } else {
                    AceExpr::Fixed(ace_expr!())
                }
            }
            Enterprise => AceExpr::Fixed(ace_expr!()),
            _ => AceExpr::Fixed(ace_expr!()),
        }
    }

    fn set_common_name_default(uid: AuthorityRef, ssc: Ssc) -> AceExpr {
        use Ssc::*;
        use opal_2::locking::authority::{ADMIN, ADMINS, USER};

        match ssc {
            OpalV2 | Ruby => {
                if ADMIN.contains(uid) {
                    AceExpr::Fixed(ace_expr!(ADMINS))
                } else if USER.contains(uid) {
                    AceExpr::Assumed(ace_expr!(ADMINS))
                } else {
                    AceExpr::Fixed(ace_expr!())
                }
            }
            OpalV1 | Opalite | PyriteV1 | PyriteV2 => {
                if USER.contains(uid) {
                    AceExpr::Assumed(ace_expr!(ADMINS))
                } else {
                    AceExpr::Fixed(ace_expr!())
                }
            }
            _ => AceExpr::Fixed(ace_expr!()),
        }
    }

    fn set_pin_default(uid: AuthorityRef, ssc: Ssc) -> AceExpr {
        use Ssc::*;
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use opal_2::locking::authority::{ADMIN, ADMINS, USER};

        match ssc {
            OpalV2 | Ruby | OpalV1 | Opalite | PyriteV1 | PyriteV2 => {
                if ADMIN.contains(uid) {
                    AceExpr::Fixed(ace_expr!(ADMINS))
                } else if USER.contains(uid) {
                    AceExpr::Assumed(ace_expr!(ADMINS))
                } else {
                    AceExpr::Fixed(ace_expr!())
                }
            }
            Enterprise => {
                if uid == ERASE_MASTER || BAND_MASTER.contains(uid) {
                    AceExpr::Fixed(ace_expr!(uid))
                } else {
                    AceExpr::Fixed(ace_expr!())
                }
            }
            _ => AceExpr::Fixed(ace_expr!()),
        }
    }
}

async fn load_ace_or_else(
    session: &Session,
    ace: Option<AceRef>,
    default: impl FnOnce() -> AceExpr,
) -> Result<AceExpr, Error> {
    let Some(ace) = ace else {
        return Ok(default());
    };
    match session.get_field(ace.boolean_expr()).await {
        Ok(expr) => Ok(AceExpr::Loaded(expr)),
        Err(TperError::MethodCallFailed(MethodStatus::NotAuthorized)) => Ok(default()),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    use sed_spec::preconfig::core as core_preconfig;

    //--------------------------------------------------------------------------
    // Authority aces
    //--------------------------------------------------------------------------

    mod authority_aces {
        use super::*;

        use Ssc::*;
        use core_preconfig::shared::authority::ANYBODY;
        use enterprise::locking::authority::{BAND_MASTER, ERASE_MASTER};
        use opal_2::locking::ace::{ADMINS_SET_COMMON_NAME, USER_SET_COMMON_NAME};
        use opal_2::locking::authority::{ADMIN, USER};

        #[rstest]
        #[case::opal_anybody(OpalV2, ANYBODY, None)]
        #[case::opal_admin1(OpalV2, ADMIN.get(0).unwrap(), Some(ADMINS_SET_COMMON_NAME))]
        #[case::opal_admin2(OpalV2, ADMIN.get(1).unwrap(), Some(ADMINS_SET_COMMON_NAME))]
        #[case::opal_user1(OpalV2, USER.get(0).unwrap(), Some(USER_SET_COMMON_NAME.get(0).unwrap()))]
        #[case::opal_user2(OpalV2, USER.get(1).unwrap(), Some(USER_SET_COMMON_NAME.get(1).unwrap()))]
        #[case::pyrite_anybody(PyriteV2, ANYBODY, None)]
        #[case::pyrite_admin1(PyriteV2, ADMIN.get(0).unwrap(), None)]
        #[case::pyrite_admin2(PyriteV2, ADMIN.get(1).unwrap(), None)]
        #[case::pyrite_user1(PyriteV2, USER.get(0).unwrap(), Some(USER_SET_COMMON_NAME.get(0).unwrap()))]
        #[case::pyrite_user2(PyriteV2, USER.get(1).unwrap(), Some(USER_SET_COMMON_NAME.get(1).unwrap()))]
        #[case::enterprise_anybody(Enterprise, ANYBODY, None)]
        #[case::enterprise_em(Enterprise, ERASE_MASTER, None)]
        #[case::enterprise_bm0(Enterprise, BAND_MASTER.get(0).unwrap(), None)]
        #[case::enterprise_bm1(Enterprise, BAND_MASTER.get(1).unwrap(), None)]
        fn set_common_name_ref(#[case] ssc: Ssc, #[case] authority: AuthorityRef, #[case] ace: Option<AceRef>) {
            assert_eq!(AuthorityAces::set_common_name_ref(authority, ssc), ace);
        }

        #[rstest]
        fn set_pin_ref() {
            todo!()
        }

        #[rstest]
        fn set_enabled_default() {
            todo!()
        }

        #[rstest]
        fn set_common_name_default() {
            todo!()
        }

        #[rstest]
        fn set_pin_default() {
            todo!()
        }
    }
}
