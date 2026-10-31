//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::{
    objects::{AuthorityRef, LockingRangeRef},
    types::AceOperand,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessControl {
    pub global: GlobalAces,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangeAces>,
    pub authorities: BTreeMap<AuthorityRef, UserAces>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalAces {
    read_ace_table: AceExpr,
    read_authority_table: AceExpr,
    set_mbr_done: AceExpr,
    read_data_store: AceExpr,
    write_data_store: AceExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangeAces {
    read_start_to_active_key: AceExpr,
    set_read_locked: AceExpr,
    set_write_locked: AceExpr,
    erase: AceExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAces {
    set_common_name: AceExpr,
    set_pin: AceExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AceExpr {
    Loaded(Vec<AceOperand>),
    Assumed(Vec<AceOperand>),
}
