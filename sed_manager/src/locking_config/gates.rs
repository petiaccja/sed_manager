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
pub struct Gates {
    pub global: GlobalGates,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangeGates>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityGates>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalGates {
    read_ace_table: bool,
    read_authority_table: bool,
    set_mbr_enabled: bool,
    set_mbr_done: bool,
    upload_mbr: bool,
    read_data_store: bool,
    write_data_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangeGates {
    read_start_to_active_key: bool,
    set_read_locked: bool,
    set_write_locked: bool,
    erase: bool,
    set_start_to_lor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityGates {
    set_enabled: bool,
    set_common_name: bool,
    set_pin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AceExpr {
    Loaded(Vec<AceOperand>),
    Assumed(Vec<AceOperand>),
    Fixed(Vec<AceOperand>),
}
