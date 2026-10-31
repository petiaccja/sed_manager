//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::{
    objects::{AuthorityRef, LockingRangeRef},
    types::AceOperand,
};
use sed_tper::Session;

use crate::{Error, Permissions};

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
pub struct AccessControl {
    pub global: GlobalAces,
    pub mbr: Option<MbrAces>,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangeAces>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityAces>,
}

impl AccessControl {
    pub async fn load(session: &Session) -> Result<Self, Error> {
        let locking_ranges_refs = session.next(None, None).await?;
        let mut locking_ranges = BTreeMap::new();
        for uid in locking_ranges_refs {
            locking_ranges.insert(uid, LockingRangeAces::load(session, uid).await?);
        }

        let authorities_refs = session.next(None, None).await?;
        let mut authorities = BTreeMap::new();
        for uid in authorities_refs {
            authorities.insert(uid, AuthorityAces::load(session, uid).await?);
        }

        Ok(Self {
            global: GlobalAces::load(session).await?,
            mbr: Some(MbrAces::load(session).await?),
            locking_ranges,
            authorities,
        })
    }

    pub fn evaluate(&self, authority: AuthorityRef) -> Permissions {
        todo!()
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityAces {
    pub set_enabled: AceExpr,
    pub set_common_name: AceExpr,
    pub set_pin: AceExpr,
}

impl AuthorityAces {
    pub async fn load(session: &Session, uid: AuthorityRef) -> Result<Self, Error> {
        Ok(Self { set_enabled: todo!(), set_common_name: todo!(), set_pin: todo!() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AceExpr {
    Loaded(Vec<AceOperand>),
    Assumed(Vec<AceOperand>),
    Fixed(Vec<AceOperand>),
}
