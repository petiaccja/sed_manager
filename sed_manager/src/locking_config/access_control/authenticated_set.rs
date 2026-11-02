//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_spec::objects::{AuthorityRef, EvalAceExpr, get_authority_class};
use smallvec::{SmallVec, smallvec};

/// The authorities that are considered authenticated in a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedSet(SmallVec<[AuthorityRef; 3]>);

impl AuthenticatedSet {
    /// Construct the authenticated set from the `authority`, its class (if any),
    /// and the `Anybody` authority. These must be authenticated anytime
    /// `authority` is authenticated.
    ///
    /// The class is derived from the preconfiguration rather than read from the
    /// device, because authorities may not have permission to read their own class.
    pub fn of(authority: AuthorityRef) -> Self {
        use sed_spec::preconfig::core::shared::authority::ANYBODY;

        let mut authenticated = smallvec![authority];

        if authority != ANYBODY {
            authenticated.push(ANYBODY);
        }
        if let Some(class) = get_authority_class(authority) {
            authenticated.push(class);
        }

        Self(authenticated)
    }

    /// Evaluates the ACE `expr` with the authenticated authorities.
    ///
    /// Invalid expressions can never be satisfied on the device, so they
    /// evaluate to `false`.
    pub fn allows(&self, expr: impl EvalAceExpr) -> bool {
        expr.eval(self.iter()).unwrap_or(false)
    }

    /// Iterate over the authenticated authorities.
    pub fn iter(&self) -> impl Iterator<Item = AuthorityRef> + '_ {
        self.0.iter().copied()
    }
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
    use sed_spec::types::AceOperand;

    use super::*;

    fn admin(n: usize) -> AuthorityRef {
        ADMIN.get(n).unwrap()
    }

    fn user(n: usize) -> AuthorityRef {
        USER.get(n).unwrap()
    }

    fn band_master(n: usize) -> AuthorityRef {
        BAND_MASTER.get(n).unwrap()
    }

    #[rstest]
    #[case::anybody(ANYBODY, vec![ANYBODY])]
    #[case::class(USERS, vec![USERS, ANYBODY])]
    #[case::user(user(1), vec![user(1), USERS, ANYBODY])]
    #[case::admin(admin(1), vec![admin(1), ADMINS, ANYBODY])]
    #[case::band_master(band_master(0), vec![band_master(0), BAND_MASTERS, ANYBODY])]
    #[case::without_class(ERASE_MASTER, vec![ERASE_MASTER, ANYBODY])]
    fn of(#[case] authority: AuthorityRef, #[case] expected: Vec<AuthorityRef>) {
        let actual: BTreeSet<_> = AuthenticatedSet::of(authority).iter().collect();
        let expected: BTreeSet<_> = expected.into_iter().collect();
        assert_that!(actual, container_eq(expected));
    }

    #[rstest]
    #[case::via_anybody(ace_expr!(ANYBODY), user(1), true)]
    #[case::via_class(ace_expr!(USERS), user(1), true)]
    #[case::via_self(ace_expr!((user(1))), user(1), true)]
    #[case::unrelated(ace_expr!(BAND_MASTERS), user(1), false)]
    #[case::invalid(ace_expr!((user(1)) ||), user(1), false)]
    #[case::empty(ace_expr!(), user(1), false)]
    fn allows(#[case] expr: Vec<AceOperand>, #[case] authority: AuthorityRef, #[case] expected: bool) {
        assert_eq!(AuthenticatedSet::of(authority).allows(&expr), expected);
    }
}
