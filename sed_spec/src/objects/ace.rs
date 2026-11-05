//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::collections::HashSet;

use sed_packet::ObjectUid;
use sed_spec_macros::{DetokenizeStruct, FieldList, Object, TokenizeField, TokenizeStruct};

use crate::objects::{AceRef, AuthorityRef};
use crate::preconfig::core::shared::table_id;
use crate::types::{AceOperand, BooleanOp};

/// Do not use this macro directly. See [ace_expr!].
#[macro_export]
macro_rules! ace_operand {
    (||) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::Or)
    };
    (&&) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::And)
    };
    (!) => {
        ::sed_spec::types::AceOperand::BooleanOp(::sed_spec::types::BooleanOp::Not)
    };
    ($authority:expr) => {
        ::sed_spec::types::AceOperand::Authority($authority)
    };
}

/// Constructs an ACE boolean expression using a natural reverse Polish notation
/// syntax.
///
/// Example:
/// ```
/// # use sed_spec::preconfig::core::shared::authority::ADMINS;
/// # use sed_spec::preconfig::core::shared::authority::SID;
/// # use sed_spec::objects::ace_expr;
/// # use sed_spec::objects::EvalAceExpr;
/// let expr = ace_expr!(SID ADMINS ||);
/// assert_eq!(expr.eval([SID]), Some(true));
#[macro_export]
macro_rules! ace_expr {
    ($($operand:tt)*) => {
        Vec::<$crate::types::AceOperand>::from(vec![$($crate::objects::ace_operand!($operand)),*])
    };
}

pub use ace_expr;
pub use ace_operand;

#[derive(Debug, Clone, Default, PartialEq, Eq, Object, TokenizeStruct, DetokenizeStruct, FieldList, TokenizeField)]
#[object(table = table_id::ACE)]
pub struct Ace {
    pub uid: Option<AceRef>,
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub boolean_expr: Option<Vec<AceOperand>>,
    pub columns: Option<HashSet<u16>>,
}

impl ObjectUid for Ace {
    fn uid(&self) -> Option<Self::Ref> {
        self.uid
    }

    fn set_uid(&mut self, uid: Option<Self::Ref>) {
        self.uid = uid;
    }
}

/// An ACE boolean expression written using the reverse Polish notation.
///
/// For example `User1 User2 &&` is true when both the `User1` and `User2`
/// authorities are authenticated, and false otherwise.
pub trait EvalAceExpr {
    /// Evaluate the ACE boolean expression, substituting `true` for the
    /// `authenticated` authorities, and `false` for all others.
    ///
    /// # Errors
    ///
    /// If the expression is not valid and therefore cannot be evaluated,
    /// [`None`] is returned.
    fn eval(self, authenticated: impl IntoIterator<Item = AuthorityRef>) -> Option<bool>;

    /// Checks if the expression is valid according to the reverse Polish
    /// notation.
    ///
    /// For example, `User1 User2 || ! !` is valid, but `User1 || !` is
    /// unbalanced and therefore cannot be evaluated.
    fn can_eval(self) -> bool
    where
        Self: Sized,
    {
        self.eval([]).is_some()
    }
}

impl<'me, AceExpr> EvalAceExpr for AceExpr
where
    Self: IntoIterator<Item = &'me AceOperand>,
{
    fn eval(self, authenticated: impl IntoIterator<Item = AuthorityRef>) -> Option<bool> {
        let mut stack = Vec::<bool>::new();
        let authenticated: HashSet<_> = authenticated.into_iter().collect();
        for item in self.into_iter() {
            match item {
                AceOperand::Authority(authority) => {
                    stack.push(authenticated.contains(authority));
                }
                AceOperand::BooleanOp(BooleanOp::And) => {
                    let rhs = stack.pop()?;
                    let lhs = stack.pop()?;
                    stack.push(lhs && rhs);
                }
                AceOperand::BooleanOp(BooleanOp::Or) => {
                    let rhs = stack.pop()?;
                    let lhs = stack.pop()?;
                    stack.push(lhs || rhs);
                }
                AceOperand::BooleanOp(BooleanOp::Not) => {
                    let arg = stack.pop()?;
                    stack.push(!arg);
                }
            }
        }
        if stack.len() >= 2 {
            None
        } else {
            stack.first().cloned().or(Some(false))
        }
    }
}

/// Modifies the ACE boolean expression such that `authority` alone
/// satisfies it.
///
/// If the `authority` alone already satisfies the expression, it is
/// returned without modification.
///
/// # Errors
///
/// If the original expression is not valid, [`None`] is returned.
pub fn allow_authority<'a, AceExpr>(ace_expr: AceExpr, authority: AuthorityRef) -> Option<Vec<AceOperand>>
where
    AceExpr: IntoIterator<Item = &'a AceOperand> + Clone,
{
    let already_allowed = ace_expr.clone().eval(std::iter::once(authority))?;
    if already_allowed {
        Some(ace_expr.into_iter().cloned().collect())
    } else {
        let mut new_expr: Vec<AceOperand> = ace_expr.into_iter().cloned().collect();
        new_expr.push(AceOperand::Authority(authority));
        if new_expr.len() != 1 {
            new_expr.push(AceOperand::BooleanOp(BooleanOp::Or));
        }
        Some(new_expr)
    }
}

/// Modifies the ACE boolean expression such that `authority` alone
/// does not satisfy it.
///
/// If the `authority` alone already does not satisfy the expression, it is
/// returned without modification.
///
/// # Errors
///
/// If the original expression is not valid, [`None`] is returned.
pub fn deny_authority<'a, AceExpr>(ace_expr: AceExpr, authority: AuthorityRef) -> Option<Vec<AceOperand>>
where
    AceExpr: IntoIterator<Item = &'a AceOperand>,
{
    // Input must already be normalized.
    let normalized = normalize(ace_expr)?;
    let mut stack = Vec::<Vec<AceOperand>>::new();
    let pattern = [AceOperand::from(authority)];
    for item in normalized {
        match item {
            AceOperand::Authority(authority) => {
                stack.push(vec![authority.into()]);
            }
            AceOperand::BooleanOp(BooleanOp::And) => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let op = [BooleanOp::And.into()];
                let evaled = lhs.into_iter().chain(rhs).chain(op);
                stack.push(evaled.collect());
            }
            AceOperand::BooleanOp(BooleanOp::Or) => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let op = [BooleanOp::Or.into()];
                if lhs.as_slice() == pattern {
                    stack.push(rhs);
                } else if rhs.as_slice() == pattern {
                    stack.push(lhs);
                } else {
                    let evaled = lhs.into_iter().chain(rhs).chain(op);
                    stack.push(evaled.collect());
                }
            }
            AceOperand::BooleanOp(BooleanOp::Not) => {
                let arg = stack.pop()?;
                let op = [BooleanOp::Not.into()];
                let evaled = arg.into_iter().chain(op);
                stack.push(evaled.collect());
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    let new_expr = stack.pop().unwrap_or(vec![]);
    if new_expr.as_slice() == pattern { Some(vec![]) } else { Some(new_expr) }
}

/// Replaces trivial expressions with a simpler form, like `X && X` with `X`.
///
/// # Errors
///
/// If the original expression is not valid, [`None`] is returned.
pub fn normalize<'a, AceExpr>(ace_expr: AceExpr) -> Option<Vec<AceOperand>>
where
    AceExpr: IntoIterator<Item = &'a AceOperand>,
{
    // Applies the following normalization patterns:
    // - X NOT NOT => X
    // - X X OR => X
    // - X X AND => X
    let mut stack = Vec::<Vec<AceOperand>>::new();
    for item in ace_expr.into_iter() {
        match item {
            AceOperand::Authority(authority) => {
                stack.push(vec![authority.into()]);
            }
            AceOperand::BooleanOp(BooleanOp::And) => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let op = [BooleanOp::And.into()];
                if lhs.as_slice() == rhs.as_slice() {
                    stack.push(rhs);
                } else {
                    let evaled = lhs.into_iter().chain(rhs).chain(op);
                    stack.push(evaled.collect());
                }
            }
            AceOperand::BooleanOp(BooleanOp::Or) => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                let op = [BooleanOp::Or.into()];
                if lhs.as_slice() == rhs.as_slice() {
                    stack.push(rhs);
                } else {
                    let evaled = lhs.into_iter().chain(rhs).chain(op);
                    stack.push(evaled.collect());
                }
            }
            AceOperand::BooleanOp(BooleanOp::Not) => {
                let mut arg = stack.pop()?;
                let op = [BooleanOp::Not.into()];
                if arg.last() == Some(&BooleanOp::Not.into()) {
                    arg.pop();
                    stack.push(arg);
                } else {
                    let evaled = arg.into_iter().chain(op);
                    stack.push(evaled.collect());
                }
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    Some(stack.pop().unwrap_or(vec![]))
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::*;

    const ALICE: AuthorityRef = AuthorityRef::from_half(NonZero::new(1).unwrap());
    const BOB: AuthorityRef = AuthorityRef::from_half(NonZero::new(2).unwrap());
    const CHARLIE: AuthorityRef = AuthorityRef::from_half(NonZero::new(3).unwrap());
    const DAVE: AuthorityRef = AuthorityRef::from_half(NonZero::new(4).unwrap());

    #[test]
    fn eval_ace_expr_empty() {
        let ace_expr = Vec::<AceOperand>::new();
        assert_eq!(ace_expr.eval([ALICE]), Some(false));
        assert_eq!(ace_expr.eval([]), Some(false));
    }

    #[test]
    fn eval_ace_expr_too_many_ops() {
        let ace_expr = ace_expr!(ALICE CHARLIE || ||);
        assert_eq!(ace_expr.eval([ALICE]), None);
    }

    #[test]
    fn eval_ace_expr_too_few_ops() {
        let ace_expr = ace_expr!(ALICE BOB CHARLIE ||);
        assert_eq!(ace_expr.eval([ALICE]), None);
    }

    #[test]
    fn eval_ace_expr_or() {
        let ace_expr = ace_expr!(ALICE CHARLIE BOB || ||);
        assert_eq!(ace_expr.eval([ALICE]), Some(true));
        assert_eq!(ace_expr.eval([BOB]), Some(true));
        assert_eq!(ace_expr.eval([CHARLIE]), Some(true));
        assert_eq!(ace_expr.eval([DAVE]), Some(false));
    }

    #[test]
    fn eval_ace_expr_and() {
        let ace_expr = ace_expr!(ALICE CHARLIE BOB && &&);
        assert_eq!(ace_expr.eval([ALICE]), Some(false));
        assert_eq!(ace_expr.eval([BOB]), Some(false));
        assert_eq!(ace_expr.eval([CHARLIE]), Some(false));
        assert_eq!(ace_expr.eval([ALICE, BOB, CHARLIE]), Some(true));
    }

    #[test]
    fn eval_ace_expr_not() {
        let ace_expr = ace_expr!(ALICE !);
        assert_eq!(ace_expr.eval([ALICE]), Some(false));
        assert_eq!(ace_expr.eval([BOB]), Some(true));
        assert_eq!(ace_expr.eval([]), Some(true));
    }

    #[test]
    fn allow_authority_allowed() {
        let ace_expr = ace_expr!(ALICE BOB ||);
        let allowed = allow_authority(&ace_expr, BOB);
        assert_eq!(ace_expr.as_slice(), allowed.unwrap().as_slice());
    }

    #[test]
    fn allow_authority_missing() {
        let ace_expr = ace_expr!(ALICE BOB ||);
        let allowed = allow_authority(&ace_expr, CHARLIE);
        let expected = ace_expr!(ALICE BOB || CHARLIE ||);
        assert_eq!(expected.as_slice(), allowed.unwrap().as_slice());
    }

    #[test]
    fn allow_authority_only() {
        let ace_expr = ace_expr!();
        let allowed = allow_authority(&ace_expr, CHARLIE);
        let expected = ace_expr!(CHARLIE);
        assert_eq!(expected.as_slice(), allowed.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_or_empty() {
        let ace_expr = ace_expr!();
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!();
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_or_too_many_ops() {
        let ace_expr = ace_expr!(DAVE || ||);
        let denied = deny_authority(&ace_expr, DAVE);
        assert!(denied.is_none());
    }
    #[test]
    fn deny_authority_or_too_few_ops() {
        let ace_expr = ace_expr!(DAVE DAVE);
        let denied = deny_authority(&ace_expr, DAVE);
        assert!(denied.is_none());
    }

    #[test]
    fn deny_authority_or_lhs() {
        let ace_expr = ace_expr!(DAVE ALICE ||);
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!(ALICE);
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_or_rhs() {
        let ace_expr = ace_expr!(ALICE DAVE ||);
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!(ALICE);
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_and_lhs() {
        let ace_expr = ace_expr!(DAVE ALICE &&);
        let denied = deny_authority(&ace_expr, DAVE);
        assert_eq!(ace_expr.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_and_rhs() {
        let ace_expr = ace_expr!(ALICE DAVE &&);
        let denied = deny_authority(&ace_expr, DAVE);
        assert_eq!(ace_expr.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_or_repeated() {
        let ace_expr = ace_expr!(DAVE DAVE ||);
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!();
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_and_repeated() {
        let ace_expr = ace_expr!(DAVE DAVE &&);
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!();
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }

    #[test]
    fn deny_authority_not_repeated() {
        let ace_expr = ace_expr!(DAVE ! !);
        let denied = deny_authority(&ace_expr, DAVE);
        let expected = ace_expr!();
        assert_eq!(expected.as_slice(), denied.unwrap().as_slice());
    }
}
