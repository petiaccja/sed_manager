//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::ace_expr;
use sed_spec::methods::MethodStatus;
use sed_spec::objects::{AceRef, AceRefExt, AuthorityKind, AuthorityRef, LockingRangeRef};
use sed_spec::preconfig::{enterprise, opal_2};
use sed_spec::types::AceOperand;
use sed_tper::{Error as TperError, Session};
use smallvec::SmallVec;

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
}
