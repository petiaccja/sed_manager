//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::objects::{AuthorityRef, LockingRangeRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permissions {
    pub global: GlobalPermissions,
    pub mbr: Option<MbrPermissions>,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangePermissions>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityPermissions>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalPermissions {
    pub read_ace_table: bool,
    pub set_ace_expressions: bool,
    pub read_authority_table: bool,
    pub read_data_store: bool,
    pub write_data_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbrPermissions {
    pub set_enabled: bool,
    pub set_done: bool,
    pub upload: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangePermissions {
    pub read_start_to_active_key: bool,
    pub set_read_locked: bool,
    pub set_write_locked: bool,
    pub erase: Option<bool>,
    pub set_start_to_lor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityPermissions {
    pub set_enabled: bool,
    pub set_common_name: bool,
    pub set_pin: bool,
}
