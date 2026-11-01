//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::objects::{AuthorityRef, LockingRangeRef};

use crate::locking_config::access_control::aces::{Aces, AuthorityAces, GlobalAces, LockingRangeAces, MbrAces};
use crate::locking_config::access_control::authenticated_set::AuthenticatedSet;

/// The permissions of an authority or set of authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permissions {
    pub global: GlobalPermissions,
    pub mbr: Option<MbrPermissions>,
    pub locking_ranges: BTreeMap<LockingRangeRef, LockingRangePermissions>,
    pub authorities: BTreeMap<AuthorityRef, AuthorityPermissions>,
}

impl Permissions {
    /// The permissions the `authenticated` authorities have according to the
    /// `aces`.
    pub fn of(authenticated: &AuthenticatedSet, aces: &Aces) -> Self {
        Self {
            global: GlobalPermissions::of(authenticated, &aces.global),
            mbr: aces.mbr.as_ref().map(|mbr| MbrPermissions::of(authenticated, mbr)),
            locking_ranges: aces
                .locking_ranges
                .iter()
                .map(|(uid, aces)| (*uid, LockingRangePermissions::of(authenticated, aces)))
                .collect(),
            authorities: aces
                .authorities
                .iter()
                .map(|(uid, aces)| (*uid, AuthorityPermissions::of(authenticated, aces)))
                .collect(),
        }
    }

    /// The effective permissions of `authority` according to the `aces`, which
    /// include the permissions of its class and `Anybody`.
    pub fn effective_of(authority: AuthorityRef, aces: &Aces) -> Self {
        Self::of(&AuthenticatedSet::of(authority), aces)
    }
}

/// The permissions to access object controlled by global ACEs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalPermissions {
    pub read_ace_table: bool,
    pub set_ace_expressions: bool,
    pub read_authority_table: bool,
    pub read_data_store: bool,
    pub write_data_store: bool,
}

impl GlobalPermissions {
    pub fn of(authenticated: &AuthenticatedSet, aces: &GlobalAces) -> Self {
        Self {
            read_ace_table: authenticated.allows(&aces.read_ace_table),
            set_ace_expressions: authenticated.allows(&aces.set_ace_expressions),
            read_authority_table: authenticated.allows(&aces.read_authority_table),
            read_data_store: authenticated.allows(&aces.read_data_store),
            write_data_store: authenticated.allows(&aces.write_data_store),
        }
    }
}

/// The permissions to access the `MBR` and `MBRControl` tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbrPermissions {
    pub set_enabled: bool,
    pub set_done: bool,
    pub upload: bool,
}

impl MbrPermissions {
    pub fn of(authenticated: &AuthenticatedSet, aces: &MbrAces) -> Self {
        let set_enable_to_dor = authenticated.allows(&aces.set_enable_to_dor);
        Self {
            set_enabled: set_enable_to_dor,
            // `Done` is covered by both ACEs.
            set_done: set_enable_to_dor || authenticated.allows(&aces.set_done),
            upload: authenticated.allows(&aces.upload),
        }
    }
}

/// The permissions to access a particular locking range object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockingRangePermissions {
    pub read_start_to_active_key: bool,
    pub set_read_locked: bool,
    pub set_write_locked: bool,
    pub erase: Option<bool>,
    pub set_start_to_lor: bool,
}

impl LockingRangePermissions {
    pub fn of(authenticated: &AuthenticatedSet, aces: &LockingRangeAces) -> Self {
        let set_start_to_lor = authenticated.allows(&aces.set_start_to_lor);
        Self {
            read_start_to_active_key: authenticated.allows(&aces.read_start_to_active_key),
            // `ReadLocked` and `WriteLocked` are also covered by `set_start_to_lor`.
            set_read_locked: set_start_to_lor || authenticated.allows(&aces.set_read_locked),
            set_write_locked: set_start_to_lor || authenticated.allows(&aces.set_write_locked),
            erase: aces.erase.as_ref().map(|erase| authenticated.allows(erase)),
            set_start_to_lor,
        }
    }
}

/// The permissions to access a particular authority object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityPermissions {
    pub set_enabled: bool,
    pub set_common_name: bool,
    pub set_pin: bool,
}

impl AuthorityPermissions {
    pub fn of(authenticated: &AuthenticatedSet, aces: &AuthorityAces) -> Self {
        Self {
            set_enabled: authenticated.allows(&aces.set_enabled),
            set_common_name: authenticated.allows(&aces.set_common_name),
            set_pin: authenticated.allows(&aces.set_pin),
        }
    }
}

#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::rstest;
    use sed_spec::objects::ace_expr;
    use sed_spec::preconfig::opal_2::locking::authority::{ADMIN, ADMINS, USER};
    use sed_spec::preconfig::opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

    use crate::locking_config::access_control::ace_expr::AceExpr;

    use super::*;

    fn admin(n: usize) -> AuthorityRef {
        ADMIN.get(n - 1).unwrap()
    }

    fn user(n: usize) -> AuthorityRef {
        USER.get(n - 1).unwrap()
    }

    fn authenticated(authority: AuthorityRef) -> AuthenticatedSet {
        AuthenticatedSet::of(authority)
    }

    /// An expression that only `authority` satisfies.
    fn only(authority: AuthorityRef) -> AceExpr {
        AceExpr::Loaded(ace_expr!((authority)))
    }

    /// An expression that nobody satisfies.
    fn nobody() -> AceExpr {
        AceExpr::Fixed(ace_expr!())
    }

    //--------------------------------------------------------------------------
    // Global permissions
    //--------------------------------------------------------------------------

    fn global_aces() -> GlobalAces {
        GlobalAces {
            read_ace_table: only(user(1)),
            set_ace_expressions: only(user(2)),
            read_authority_table: only(user(3)),
            read_data_store: only(user(4)),
            write_data_store: only(user(5)),
        }
    }

    fn global_permissions(selected: usize) -> GlobalPermissions {
        GlobalPermissions {
            read_ace_table: selected == 1,
            set_ace_expressions: selected == 2,
            read_authority_table: selected == 3,
            read_data_store: selected == 4,
            write_data_store: selected == 5,
        }
    }

    #[rstest]
    #[case::read_ace_table(1)]
    #[case::set_ace_expressions(2)]
    #[case::read_authority_table(3)]
    #[case::read_data_store(4)]
    #[case::write_data_store(5)]
    #[case::none(6)]
    fn evaluate_global_permissions(#[case] selected: usize) {
        let permissions = GlobalPermissions::of(&authenticated(user(selected)), &global_aces());
        assert_that!(permissions, eq(&global_permissions(selected)));
    }

    //--------------------------------------------------------------------------
    // MBR permissions
    //--------------------------------------------------------------------------

    fn mbr_aces() -> MbrAces {
        MbrAces { set_enable_to_dor: only(user(1)), set_done: only(user(2)), upload: only(user(3)) }
    }

    #[rstest]
    #[case::set_enable_to_dor_grants_done(1, MbrPermissions { set_enabled: true, set_done: true, upload: false })]
    #[case::set_done_alone(2, MbrPermissions { set_enabled: false, set_done: true, upload: false })]
    #[case::upload(3, MbrPermissions { set_enabled: false, set_done: false, upload: true })]
    #[case::none(4, MbrPermissions { set_enabled: false, set_done: false, upload: false })]
    fn evaluate_mbr_persmissions(#[case] selected: usize, #[case] expected: MbrPermissions) {
        let permissions = MbrPermissions::of(&authenticated(user(selected)), &mbr_aces());
        assert_that!(permissions, eq(&expected));
    }

    //--------------------------------------------------------------------------
    // Locking range persmissions
    //--------------------------------------------------------------------------

    fn locking_range_aces() -> LockingRangeAces {
        LockingRangeAces {
            read_start_to_active_key: only(user(1)),
            set_read_locked: only(user(2)),
            set_write_locked: only(user(3)),
            erase: Some(only(user(4))),
            set_start_to_lor: only(user(5)),
        }
    }

    fn locking_range_permissions(
        read_start_to_active_key: bool,
        set_read_locked: bool,
        set_write_locked: bool,
        erase: bool,
        set_start_to_lor: bool,
    ) -> LockingRangePermissions {
        LockingRangePermissions {
            read_start_to_active_key,
            set_read_locked,
            set_write_locked,
            erase: Some(erase),
            set_start_to_lor,
        }
    }

    #[rstest]
    #[case::read_start_to_active_key(1, locking_range_permissions(true, false, false, false, false))]
    #[case::set_read_locked_alone(2, locking_range_permissions(false, true, false, false, false))]
    #[case::set_write_locked_alone(3, locking_range_permissions(false, false, true, false, false))]
    #[case::erase(4, locking_range_permissions(false, false, false, true, false))]
    #[case::set_start_to_lor_grants_locked(5, locking_range_permissions(false, true, true, false, true))]
    #[case::none(6, locking_range_permissions(false, false, false, false, false))]
    fn evaluate_locking_range_permissions(#[case] selected: usize, #[case] expected: LockingRangePermissions) {
        let permissions = LockingRangePermissions::of(&authenticated(user(selected)), &locking_range_aces());
        assert_that!(permissions, eq(&expected));
    }

    #[test]
    fn evaluate_locking_range_permissions_no_erase() {
        let aces = LockingRangeAces { erase: None, ..locking_range_aces() };
        assert_eq!(LockingRangePermissions::of(&authenticated(user(4)), &aces).erase, None);
    }

    //--------------------------------------------------------------------------
    // Authority permissions
    //--------------------------------------------------------------------------

    fn authority_aces() -> AuthorityAces {
        AuthorityAces { set_enabled: only(user(1)), set_common_name: only(user(2)), set_pin: only(user(3)) }
    }

    #[rstest]
    #[case::set_enabled(1, AuthorityPermissions { set_enabled: true, set_common_name: false, set_pin: false })]
    #[case::set_common_name(2, AuthorityPermissions { set_enabled: false, set_common_name: true, set_pin: false })]
    #[case::set_pin(3, AuthorityPermissions { set_enabled: false, set_common_name: false, set_pin: true })]
    #[case::none(4, AuthorityPermissions { set_enabled: false, set_common_name: false, set_pin: false })]
    fn evaluate_authority_permissions(#[case] selected: usize, #[case] expected: AuthorityPermissions) {
        let permissions = AuthorityPermissions::of(&authenticated(user(selected)), &authority_aces());
        assert_that!(permissions, eq(&expected));
    }

    //--------------------------------------------------------------------------
    // All permissions
    //--------------------------------------------------------------------------

    #[test]
    fn evaluate_all_permissions() {
        let aces = Aces {
            global: global_aces(),
            mbr: Some(mbr_aces()),
            locking_ranges: [
                (GLOBAL_RANGE, locking_range_aces()),
                (RANGE.get(0).unwrap(), LockingRangeAces { set_read_locked: nobody(), ..locking_range_aces() }),
            ]
            .into(),
            authorities: [(user(1), authority_aces())].into(),
        };

        let permissions = Permissions::of(&authenticated(user(2)), &aces);

        let expected = Permissions {
            global: global_permissions(2),
            mbr: Some(MbrPermissions { set_enabled: false, set_done: true, upload: false }),
            locking_ranges: [
                (GLOBAL_RANGE, locking_range_permissions(false, true, false, false, false)),
                (RANGE.get(0).unwrap(), locking_range_permissions(false, false, false, false, false)),
            ]
            .into(),
            authorities: [(
                user(1),
                AuthorityPermissions { set_enabled: false, set_common_name: true, set_pin: false },
            )]
            .into(),
        };

        assert_that!(permissions, eq(&expected));
    }

    #[test]
    fn evaluate_all_permissions_no_mbr() {
        let aces =
            Aces { global: global_aces(), mbr: None, locking_ranges: BTreeMap::new(), authorities: BTreeMap::new() };
        let permissions = Permissions::of(&authenticated(user(1)), &aces);
        assert_that!(permissions.mbr, none());
    }

    #[test]
    fn evaluate_all_permissions_via_class() {
        // `ADMINS` only matches through the class, so this fails if
        // `effective_of` doesn't compute the authenticated set.
        let aces = Aces {
            global: GlobalAces { read_ace_table: AceExpr::Assumed(ace_expr!((ADMINS))), ..global_aces() },
            mbr: None,
            locking_ranges: BTreeMap::new(),
            authorities: BTreeMap::new(),
        };
        assert!(Permissions::effective_of(admin(1), &aces).global.read_ace_table);
        assert!(!Permissions::effective_of(user(1), &aces).global.read_ace_table);
    }
}
