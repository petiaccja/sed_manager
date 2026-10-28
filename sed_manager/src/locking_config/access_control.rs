//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::{collections::HashMap, iter};

use sed_spec::{
    objects::{AceExpr, AuthorityRef, LockingRangeRef},
    preconfig::core::shared::authority::ANYBODY,
    types::AceOperand,
};

/// Permissions to modify certain aspects of a locking range.
pub enum LockingRangePermission {
    SetAll,
    SetReadLocked,
    SetWriteLocked,
    GenKey,
}

/// The most important access control elements for configuring the device.
#[derive(Debug, PartialEq, Eq)]
pub struct AccessControl {
    pub locking_ranges: HashMap<LockingRangeRef, LockingRangeAccess>,
    pub mbr: MbrAccess,
}

/// The most important access control elements related to a locking range.
#[derive(Debug, PartialEq, Eq)]
pub struct LockingRangeAccess {
    pub set_all: Vec<AceOperand>,
    pub set_read_locked: Vec<AceOperand>,
    pub set_write_locked: Vec<AceOperand>,
    pub gen_key: Vec<AceOperand>,
}

impl LockingRangeAccess {
    /// Evaluates if the `authority` alone has the `permission` to this locking
    /// range. Implied permissions through the `Anybody` authority or the
    /// `authority`'s class are not considered.
    ///
    /// If the ACE expression is invalid, we assume that nobody has permission
    /// from the device's perspective.
    pub fn has_permission_alone(&self, authority: AuthorityRef, permission: LockingRangePermission) -> bool {
        self.has_permission_combined([authority], permission)
    }

    /// Evaluates if the `authority` has effective the `permission` to this locking
    /// range. `true` is also returned if the `authority` has implied permission
    /// though the `Anybody` authority or its class.
    ///
    /// If the ACE expression is invalid, we assume that nobody has permission
    /// from the device's perspective.
    pub fn has_permission_effective(
        &self,
        authority: AuthorityRef,
        class: Option<AuthorityRef>,
        permission: LockingRangePermission,
    ) -> bool {
        let authorities = iter::once(ANYBODY).chain(class).chain(iter::once(authority));
        self.has_permission_combined(authorities, permission)
    }

    fn has_permission_combined(
        &self,
        authorities: impl IntoIterator<Item = AuthorityRef>,
        permission: LockingRangePermission,
    ) -> bool {
        use LockingRangePermission::*;
        let ace = match permission {
            SetAll => &self.set_all,
            SetReadLocked => &self.set_read_locked,
            SetWriteLocked => &self.set_write_locked,
            GenKey => &self.gen_key,
        };
        ace.eval(authorities).unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MbrAccess {
    pub set_done: Vec<AceOperand>,
}

impl MbrAccess {
    /// Evaluates if the `authority` alone has the `permission` to set the
    /// `Done` field of `MbrControl`. Implied permissions through the `Anybody`
    /// authority or the `authority`'s class are not considered.
    ///
    /// If the ACE expression is invalid, we assume that nobody has permission
    /// from the device's perspective.
    pub fn has_permission_alone(&self, authority: AuthorityRef) -> bool {
        self.set_done.eval([authority]).unwrap_or(false)
    }

    /// Evaluates if the `authority` alone has the `permission` to set the
    /// `Done` field of `MbrControl`. `true` is also returned if the `authority`
    /// has implied permission though the `Anybody` authority or its class.
    ///
    /// If the ACE expression is invalid, we assume that nobody has permission
    /// from the device's perspective.
    pub fn has_permission_effective(&self, authority: AuthorityRef, class: Option<AuthorityRef>) -> bool {
        let authorities = iter::once(ANYBODY).chain(class).chain(iter::once(authority));
        self.set_done.eval(authorities).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use rstest::rstest;
    use sed_spec::objects::ace_expr;

    use super::*;

    // Chosen to not collide with ANYBODY or other preconfigured authorities.
    const ALICE: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0001).unwrap());
    const BOB: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0002).unwrap());
    const CHARLIE: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0003).unwrap());
    const DAVE: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0004).unwrap());
    const CLASS: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0005).unwrap());
    const OTHER_CLASS: AuthorityRef = AuthorityRef::from_half(NonZero::new(0xFF00_0006).unwrap());

    fn make_uniform_locking_range_access(ace: Vec<AceOperand>) -> LockingRangeAccess {
        LockingRangeAccess {
            set_all: ace.clone(),
            set_read_locked: ace.clone(),
            set_write_locked: ace.clone(),
            gen_key: ace,
        }
    }

    fn make_distinct_locking_range_access() -> LockingRangeAccess {
        LockingRangeAccess {
            set_all: ace_expr!(ALICE),
            set_read_locked: ace_expr!(BOB),
            set_write_locked: ace_expr!(CHARLIE),
            gen_key: ace_expr!(DAVE),
        }
    }

    //--------------------------------------------------------------------------
    // LockingRangeAccess
    //--------------------------------------------------------------------------

    #[rstest]
    #[case::set_all(LockingRangePermission::SetAll, ALICE)]
    #[case::set_read_locked(LockingRangePermission::SetReadLocked, BOB)]
    #[case::set_write_locked(LockingRangePermission::SetWriteLocked, CHARLIE)]
    #[case::gen_key(LockingRangePermission::GenKey, DAVE)]
    fn locking_range_has_permission_alone_selects_ace(
        #[case] permission: LockingRangePermission,
        #[case] allowed: AuthorityRef,
        #[values(ALICE, BOB, CHARLIE, DAVE)] authority: AuthorityRef,
    ) {
        let access = make_distinct_locking_range_access();
        assert_eq!(access.has_permission_alone(authority, permission), authority == allowed);
    }

    #[rstest]
    #[case::set_all(LockingRangePermission::SetAll, ALICE)]
    #[case::set_read_locked(LockingRangePermission::SetReadLocked, BOB)]
    #[case::set_write_locked(LockingRangePermission::SetWriteLocked, CHARLIE)]
    #[case::gen_key(LockingRangePermission::GenKey, DAVE)]
    fn locking_range_has_permission_effective_selects_ace(
        #[case] permission: LockingRangePermission,
        #[case] allowed: AuthorityRef,
        #[values(ALICE, BOB, CHARLIE, DAVE)] authority: AuthorityRef,
    ) {
        let access = make_distinct_locking_range_access();
        assert_eq!(access.has_permission_effective(authority, None, permission), authority == allowed);
    }

    #[rstest]
    #[case::authority_allowed(ace_expr!(ALICE), ALICE, true)]
    #[case::authority_denied(ace_expr!(BOB), ALICE, false)]
    #[case::anybody_not_implied(ace_expr!(ANYBODY), ALICE, false)]
    #[case::invalid_expr(ace_expr!(ALICE ||), ALICE, false)]
    fn locking_range_has_permission_alone(
        #[case] ace: Vec<AceOperand>,
        #[case] authority: AuthorityRef,
        #[case] expected: bool,
    ) {
        let access = make_uniform_locking_range_access(ace);
        assert_eq!(access.has_permission_alone(authority, LockingRangePermission::SetAll), expected);
    }

    #[rstest]
    #[case::authority_allowed(ace_expr!(ALICE), ALICE, None, true)]
    #[case::authority_denied(ace_expr!(BOB), ALICE, Some(CLASS), false)]
    #[case::anybody_implied(ace_expr!(ANYBODY), ALICE, None, true)]
    #[case::class_implied(ace_expr!(CLASS), ALICE, Some(CLASS), true)]
    #[case::class_missing(ace_expr!(CLASS), ALICE, None, false)]
    #[case::class_mismatch(ace_expr!(CLASS), ALICE, Some(OTHER_CLASS), false)]
    #[case::all_combined(ace_expr!(ANYBODY CLASS ALICE && &&), ALICE, Some(CLASS), true)]
    #[case::invalid_expr(ace_expr!(ALICE ||), ALICE, Some(CLASS), false)]
    fn locking_range_has_permission_effective(
        #[case] ace: Vec<AceOperand>,
        #[case] authority: AuthorityRef,
        #[case] class: Option<AuthorityRef>,
        #[case] expected: bool,
    ) {
        let access = make_uniform_locking_range_access(ace);
        assert_eq!(access.has_permission_effective(authority, class, LockingRangePermission::SetAll), expected);
    }

    //--------------------------------------------------------------------------
    // MbrAccess
    //--------------------------------------------------------------------------

    #[rstest]
    #[case::authority_allowed(ace_expr!(ALICE), ALICE, true)]
    #[case::authority_denied(ace_expr!(BOB), ALICE, false)]
    #[case::anybody_not_implied(ace_expr!(ANYBODY), ALICE, false)]
    #[case::invalid_expr(ace_expr!(ALICE ||), ALICE, false)]
    fn mbr_has_permission_alone(#[case] ace: Vec<AceOperand>, #[case] authority: AuthorityRef, #[case] expected: bool) {
        let access = MbrAccess { set_done: ace };
        assert_eq!(access.has_permission_alone(authority), expected);
    }

    #[rstest]
    #[case::authority_allowed(ace_expr!(ALICE), ALICE, None, true)]
    #[case::authority_denied(ace_expr!(BOB), ALICE, Some(CLASS), false)]
    #[case::anybody_implied(ace_expr!(ANYBODY), ALICE, None, true)]
    #[case::class_implied(ace_expr!(CLASS), ALICE, Some(CLASS), true)]
    #[case::class_missing(ace_expr!(CLASS), ALICE, None, false)]
    #[case::class_mismatch(ace_expr!(CLASS), ALICE, Some(OTHER_CLASS), false)]
    #[case::all_combined(ace_expr!(ANYBODY CLASS ALICE && &&), ALICE, Some(CLASS), true)]
    #[case::invalid_expr(ace_expr!(ALICE ||), ALICE, Some(CLASS), false)]
    fn mbr_has_permission_effective(
        #[case] ace: Vec<AceOperand>,
        #[case] authority: AuthorityRef,
        #[case] class: Option<AuthorityRef>,
        #[case] expected: bool,
    ) {
        let access = MbrAccess { set_done: ace };
        assert_eq!(access.has_permission_effective(authority, class), expected);
    }
}
