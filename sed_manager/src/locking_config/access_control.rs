//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::BTreeMap;

use sed_spec::{
    objects::{AuthorityRef, EvalAceExpr, LockingRangeRef},
    types::AceOperand,
};
use sed_tper::Session;
use smallvec::{SmallVec, smallvec};

use crate::{AuthorityPermissions, Error, GlobalPermissions, LockingRangePermissions, MbrPermissions, Permissions};

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
        Permissions {
            global: self.global.evaluate(authority),
            mbr: self.mbr.as_ref().map(|mbr| mbr.evaluate(authority)),
            locking_ranges: self.locking_ranges.iter().map(|(uid, aces)| (*uid, aces.evaluate(authority))).collect(),
            authorities: self.authorities.iter().map(|(uid, aces)| (*uid, aces.evaluate(authority))).collect(),
        }
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

    pub fn evaluate(&self, authority: AuthorityRef) -> GlobalPermissions {
        let authenticated = authenticated_set(authority);
        GlobalPermissions {
            read_ace_table: self.read_ace_table.allows(&authenticated),
            set_ace_expressions: self.set_ace_expressions.allows(&authenticated),
            read_authority_table: self.read_authority_table.allows(&authenticated),
            read_data_store: self.read_data_store.allows(&authenticated),
            write_data_store: self.write_data_store.allows(&authenticated),
        }
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

    pub fn evaluate(&self, authority: AuthorityRef) -> MbrPermissions {
        let authenticated = authenticated_set(authority);
        let set_enable_to_dor = self.set_enable_to_dor.allows(&authenticated);
        MbrPermissions {
            set_enabled: set_enable_to_dor,
            // `Done` is covered by both ACEs.
            set_done: set_enable_to_dor || self.set_done.allows(&authenticated),
            upload: self.upload.allows(&authenticated),
        }
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

    pub fn evaluate(&self, authority: AuthorityRef) -> LockingRangePermissions {
        let authenticated = authenticated_set(authority);
        let set_start_to_lor = self.set_start_to_lor.allows(&authenticated);
        LockingRangePermissions {
            read_start_to_active_key: self.read_start_to_active_key.allows(&authenticated),
            // `ReadLocked` and `WriteLocked` are also covered by `set_start_to_lor`.
            set_read_locked: set_start_to_lor || self.set_read_locked.allows(&authenticated),
            set_write_locked: set_start_to_lor || self.set_write_locked.allows(&authenticated),
            erase: self.erase.as_ref().map(|erase| erase.allows(&authenticated)),
            set_start_to_lor,
        }
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

    pub fn evaluate(&self, authority: AuthorityRef) -> AuthorityPermissions {
        let authenticated = authenticated_set(authority);
        AuthorityPermissions {
            set_enabled: self.set_enabled.allows(&authenticated),
            set_common_name: self.set_common_name.allows(&authenticated),
            set_pin: self.set_pin.allows(&authenticated),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AceExpr {
    Loaded(Vec<AceOperand>),
    Assumed(Vec<AceOperand>),
    Fixed(Vec<AceOperand>),
}

impl AceExpr {
    /// Evaluates the expression with the `authenticated` authorities.
    ///
    /// Invalid expressions can never be satisfied on the device, so they
    /// evaluate to `false`.
    fn allows(&self, authenticated: &[AuthorityRef]) -> bool {
        self.eval(authenticated.iter().copied()).unwrap_or(false)
    }
}

impl<'a> IntoIterator for &'a AceExpr {
    type Item = &'a AceOperand;

    type IntoIter = std::slice::Iter<'a, AceOperand>;

    fn into_iter(self) -> Self::IntoIter {
        use AceExpr::*;
        match self {
            Loaded(inner) | Assumed(inner) | Fixed(inner) => inner.iter(),
        }
    }
}

impl IntoIterator for AceExpr {
    type Item = AceOperand;

    type IntoIter = std::vec::IntoIter<AceOperand>;

    fn into_iter(self) -> Self::IntoIter {
        use AceExpr::*;
        match self {
            Loaded(inner) | Assumed(inner) | Fixed(inner) => inner.into_iter(),
        }
    }
}

/// The authorities that are considered authenticated when `authority` is
/// authenticated: the authority itself, its class, and `Anybody`.
///
/// The class is derived from the preconfiguration rather than read from the
/// device, because authorities may not have permission to read their own class.
fn authenticated_set(authority: AuthorityRef) -> SmallVec<[AuthorityRef; 3]> {
    use core::shared::authority::ANYBODY;
    use enterprise::locking::authority::{BAND_MASTER, BAND_MASTERS};
    use opal_2::locking::authority::{ADMIN, ADMINS, USER, USERS};
    use sed_spec::preconfig::{core, enterprise, opal_2};

    let mut authenticated = smallvec![authority];

    if USER.contains(authority) {
        authenticated.push(USERS);
    } else if ADMIN.contains(authority) {
        authenticated.push(ADMINS);
    } else if BAND_MASTER.contains(authority) {
        authenticated.push(BAND_MASTERS);
    }

    if authority != ANYBODY {
        authenticated.push(ANYBODY);
    }

    authenticated
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::rstest;
    use sed_spec::objects::ace_expr;
    use sed_spec::preconfig::core::shared::authority::ANYBODY;
    use sed_spec::preconfig::enterprise::locking::authority::{BAND_MASTER, BAND_MASTERS, ERASE_MASTER};
    use sed_spec::preconfig::opal_2::locking::authority::{ADMIN, ADMINS, USER, USERS};
    use sed_spec::preconfig::opal_2::locking::locking::{GLOBAL_RANGE, RANGE};

    use super::*;

    fn admin(n: usize) -> AuthorityRef {
        ADMIN.get(n - 1).unwrap()
    }

    fn user(n: usize) -> AuthorityRef {
        USER.get(n - 1).unwrap()
    }

    fn band_master(n: usize) -> AuthorityRef {
        BAND_MASTER.get(n).unwrap()
    }

    /// An expression that only `authority` satisfies.
    fn only(authority: AuthorityRef) -> AceExpr {
        AceExpr::Loaded(ace_expr!((authority)))
    }

    /// An expression that nobody satisfies.
    fn nobody() -> AceExpr {
        AceExpr::Fixed(ace_expr!())
    }

    #[rstest]
    #[case::user(user(1), vec![user(1), USERS, ANYBODY])]
    #[case::admin(admin(1), vec![admin(1), ADMINS, ANYBODY])]
    #[case::band_master(band_master(0), vec![band_master(0), BAND_MASTERS, ANYBODY])]
    #[case::authority_without_class(ERASE_MASTER, vec![ERASE_MASTER, ANYBODY])]
    #[case::anybody(ANYBODY, vec![ANYBODY])]
    fn authenticated_set_includes_class_and_anybody(
        #[case] authority: AuthorityRef,
        #[case] expected: Vec<AuthorityRef>,
    ) {
        let actual: BTreeSet<_> = authenticated_set(authority).into_iter().collect();
        let expected: BTreeSet<_> = expected.into_iter().collect();
        assert_that!(actual, container_eq(expected));
    }

    #[test]
    fn authenticated_set_has_no_duplicates() {
        let authenticated = authenticated_set(user(1));
        let unique: BTreeSet<_> = authenticated.iter().collect();
        assert_eq!(authenticated.len(), unique.len());
    }

    #[rstest]
    #[case::loaded(AceExpr::Loaded(ace_expr!((user(1)))))]
    #[case::assumed(AceExpr::Assumed(ace_expr!((user(1)))))]
    #[case::fixed(AceExpr::Fixed(ace_expr!((user(1)))))]
    fn allows_ignores_source(#[case] expr: AceExpr) {
        assert!(expr.allows(&[user(1)]));
        assert!(!expr.allows(&[user(2)]));
    }

    #[rstest]
    #[case::anybody(ace_expr!((ANYBODY)), user(1), true)]
    #[case::own_class(ace_expr!((USERS)), user(1), true)]
    #[case::other_class(ace_expr!((ADMINS)), user(1), false)]
    #[case::admin_class(ace_expr!((ADMINS)), admin(2), true)]
    #[case::band_master_class(ace_expr!((BAND_MASTERS)), band_master(3), true)]
    #[case::one_of_or(ace_expr!((ADMINS) (user(1)) ||), user(1), true)]
    #[case::not_in_or(ace_expr!((ADMINS) (user(1)) ||), user(2), false)]
    #[case::invalid(ace_expr!((user(1)) ||), user(1), false)]
    #[case::empty(ace_expr!(), user(1), false)]
    fn allows_with_authenticated_set(
        #[case] expr: Vec<AceOperand>,
        #[case] authority: AuthorityRef,
        #[case] expected: bool,
    ) {
        let authenticated = authenticated_set(authority);
        assert_eq!(AceExpr::Loaded(expr).allows(&authenticated), expected);
    }

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
    fn global_evaluate_maps_each_ace(#[case] selected: usize) {
        assert_that!(global_aces().evaluate(user(selected)), eq(&global_permissions(selected)));
    }

    fn mbr_aces() -> MbrAces {
        MbrAces { set_enable_to_dor: only(user(1)), set_done: only(user(2)), upload: only(user(3)) }
    }

    #[rstest]
    #[case::set_enable_to_dor_grants_done(1, MbrPermissions { set_enabled: true, set_done: true, upload: false })]
    #[case::set_done_alone(2, MbrPermissions { set_enabled: false, set_done: true, upload: false })]
    #[case::upload(3, MbrPermissions { set_enabled: false, set_done: false, upload: true })]
    #[case::none(4, MbrPermissions { set_enabled: false, set_done: false, upload: false })]
    fn mbr_evaluate(#[case] selected: usize, #[case] expected: MbrPermissions) {
        assert_that!(mbr_aces().evaluate(user(selected)), eq(&expected));
    }

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
    fn locking_range_evaluate(#[case] selected: usize, #[case] expected: LockingRangePermissions) {
        assert_that!(locking_range_aces().evaluate(user(selected)), eq(&expected));
    }

    #[test]
    fn locking_range_evaluate_does_not_grant_erase_via_set_start_to_lor() {
        // Erase is the only per-range permission without a second path.
        let aces = LockingRangeAces { erase: Some(nobody()), ..locking_range_aces() };
        assert_eq!(aces.evaluate(user(5)).erase, Some(false));
    }

    #[test]
    fn locking_range_evaluate_erase_not_applicable() {
        let aces = LockingRangeAces { erase: None, ..locking_range_aces() };
        assert_eq!(aces.evaluate(user(4)).erase, None);
    }

    fn authority_aces() -> AuthorityAces {
        AuthorityAces { set_enabled: only(user(1)), set_common_name: only(user(2)), set_pin: only(user(3)) }
    }

    #[rstest]
    #[case::set_enabled(1, AuthorityPermissions { set_enabled: true, set_common_name: false, set_pin: false })]
    #[case::set_common_name(2, AuthorityPermissions { set_enabled: false, set_common_name: true, set_pin: false })]
    #[case::set_pin(3, AuthorityPermissions { set_enabled: false, set_common_name: false, set_pin: true })]
    #[case::none(4, AuthorityPermissions { set_enabled: false, set_common_name: false, set_pin: false })]
    fn authority_evaluate(#[case] selected: usize, #[case] expected: AuthorityPermissions) {
        assert_that!(authority_aces().evaluate(user(selected)), eq(&expected));
    }

    #[test]
    fn authority_evaluate_preconfigured_user_pin() {
        // The preconfigured `Admins OR User1` lets both User1 and admins change
        // User1's password, but not other users.
        let aces = AuthorityAces {
            set_enabled: AceExpr::Fixed(ace_expr!((ADMINS))),
            set_common_name: AceExpr::Assumed(ace_expr!((ADMINS))),
            set_pin: AceExpr::Assumed(ace_expr!((ADMINS) (user(1)) ||)),
        };
        assert!(aces.evaluate(user(1)).set_pin);
        assert!(aces.evaluate(admin(1)).set_pin);
        assert!(!aces.evaluate(user(2)).set_pin);
        assert!(!aces.evaluate(user(1)).set_enabled);
        assert!(aces.evaluate(admin(1)).set_enabled);
    }

    #[test]
    fn evaluate_maps_all_objects() {
        let range_1 = RANGE.get(0).unwrap();
        let access_control = AccessControl {
            global: global_aces(),
            mbr: Some(mbr_aces()),
            locking_ranges: BTreeMap::from([
                (GLOBAL_RANGE, locking_range_aces()),
                (range_1, LockingRangeAces { set_read_locked: nobody(), ..locking_range_aces() }),
            ]),
            authorities: BTreeMap::from([(user(1), authority_aces())]),
        };

        let permissions = access_control.evaluate(user(2));

        assert_that!(permissions.global, eq(&global_permissions(2)));
        assert_that!(permissions.mbr, some(eq(&MbrPermissions { set_enabled: false, set_done: true, upload: false })));
        assert_that!(
            permissions.locking_ranges.keys().collect::<Vec<_>>(),
            elements_are![eq(&&GLOBAL_RANGE), eq(&&range_1)]
        );
        assert!(permissions.locking_ranges[&GLOBAL_RANGE].set_read_locked);
        assert!(!permissions.locking_ranges[&range_1].set_read_locked);
        assert_that!(
            permissions.authorities.get(&user(1)),
            some(eq(&AuthorityPermissions { set_enabled: false, set_common_name: true, set_pin: false }))
        );
    }

    #[test]
    fn evaluate_without_mbr() {
        let access_control = AccessControl {
            global: global_aces(),
            mbr: None,
            locking_ranges: BTreeMap::new(),
            authorities: BTreeMap::new(),
        };
        assert_that!(access_control.evaluate(user(1)).mbr, none());
    }
}
