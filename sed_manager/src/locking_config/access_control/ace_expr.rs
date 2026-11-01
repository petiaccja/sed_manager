//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_spec::types::AceOperand;

/// An ACE boolean expression from one of the ACE objects of the device.
///
/// This object implements [`EvalAceExpr`], so you can compute its value
/// against a set of authenticated authorities.
///
/// [`EvalAceExpr`]: sed_spec::objects::EvalAceExpr
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AceExpr {
    /// The ACE expression was loaded directly from the device via `Get`.
    Loaded(Vec<AceOperand>),
    /// The ACE expression could not be loaded, so its preconfiguration defaults
    /// are assumed.
    Assumed(Vec<AceOperand>),
    /// The device does not allow changing the ACE expression, so this is its
    /// fixed preconfiguration.
    Fixed(Vec<AceOperand>),
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
