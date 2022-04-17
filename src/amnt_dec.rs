// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

pub use fpdec::{Dec, Decimal};

/// Type used for the numerical part of a Quantity.
///
/// When feature `fpdec` is off (= default), AmountT is defined as `f64` on a
/// 64-bit system or as `f32` on a 32-bit system.
///
/// When feature fpdec is activated, AmountT is defined as `Decimal`
/// (imported from crate `fpdec`).
///
/// The macro `Amnt!` can be used to convert float literals correctly to
/// `AmountT` depending on the configuration.
pub type AmountT = Decimal;

/// AmountT constant equal 0
pub const AMNT_ZERO: AmountT = Decimal::ZERO;

/// AmountT constant equal 1
pub const AMNT_ONE: AmountT = Decimal::ONE;

#[allow(non_snake_case)]
#[macro_export]
/// Converts a numeric literal to an `AmountT`.
macro_rules! Amnt {
    ($lit:literal) => {
        Dec!($lit)
    };
}

#[macro_export]
macro_rules! assert_almost_eq {
    ($x:expr, $y:expr) => {
        assert_eq!($x, $y, "{} ≉ {}", ($x), ($y));
    };
}
