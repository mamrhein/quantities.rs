// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#![doc = include_str ! ("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Div, Mul, Sub};
#[cfg(feature = "fpdec")]
pub use fpdec::{Dec, Decimal};
pub use si_prefixes::SIPrefix;
pub use unitless::{Unitless, ONE};

pub mod prelude;
mod si_prefixes;
mod unitless;

#[cfg(feature = "datavolume")]
pub mod datavolume;
#[cfg(feature = "duration")]
pub mod duration;
#[cfg(feature = "length")]
pub mod length;
#[cfg(feature = "mass")]
pub mod mass;

#[cfg(feature = "fpdec")]
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
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "64"))]
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
pub type AmountT = f64;
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "32"))]
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
pub type AmountT = f32;

#[cfg(feature = "fpdec")]
#[allow(non_snake_case)]
#[macro_export]
/// Converts a numeric literal to an `AmountT`.
macro_rules! Amnt {
    ($lit:literal) => {
        Dec!($lit)
    };
}
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "64"))]
#[allow(non_snake_case)]
#[macro_export]
/// Converts a numeric literal to an `AmountT`.
macro_rules! Amnt {
    ($lit:literal) => {
        $lit as f64
    };
}
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "32"))]
#[allow(non_snake_case)]
#[macro_export]
/// Converts a numeric literal to an `AmountT`.
macro_rules! Amnt {
    ($lit:literal) => {
        $lit as f32
    };
}

/// The abstract type of units used to define quantities.
pub trait Unit: Copy + Eq + PartialEq + Sized + Mul<AmountT> {
    /// Associated type of quantity
    type QuantityType: Quantity<UnitType = Self>;

    /// Optional unit used as reference for scaling the units.
    const REF_UNIT: Option<Self>;

    /// Returns an iterator over the variants of `Self`.
    fn iter<'a>() -> core::slice::Iter<'a, Self>;

    /// Returns `Some(unit)` where `unit.symbol()` == `symbol`, or `None` if
    /// there is no such unit.
    fn from_symbol(symbol: &str) -> Option<Self> {
        for unit in Self::iter() {
            if unit.symbol() == symbol {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns `Some(unit)` where `unit.scale()` == `Some(amnt)`, or `None` if
    /// there is no such unit.
    fn from_scale(amnt: AmountT) -> Option<Self> {
        for unit in Self::iter() {
            if unit.scale() == Some(amnt) {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns the name of `self`.
    fn name(&self) -> &'static str;

    /// Returns the symbol used to represent `self`.
    fn symbol(&self) -> &'static str;

    /// Returns the SI prefix of `self`, or None is `self` is not a SI unit.
    fn si_prefix(&self) -> Option<SIPrefix>;

    /// Returns `true` if `self` is the reference unit of its unit type.
    #[inline]
    fn is_ref_unit(&self) -> bool {
        Self::REF_UNIT == Some(*self)
    }

    /// Returns `Some(factor)` so that `factor` * `Self::REFUNIT` == 1 * `self`,
    /// or `None` if `Self::REF_UNIT` is `None`.
    fn scale(&self) -> Option<AmountT>;

    /// Returns `Some(factor)` so that `factor` * `other` == 1 * `self`, or
    /// `None` if `Self::REF_UNIT` is `None`.
    fn ratio(&self, other: &Self) -> Option<AmountT> {
        match (self.scale(), other.scale()) {
            (Some(from), Some(into)) => Some(from / into),
            _ => None,
        }
    }
}

/// The abstract type of quantities.
pub trait Quantity:
    Copy + Sized + Add<Self> + Sub<Self> + Div<Self> + Mul<AmountT>
{
    /// Associated type of unit
    type UnitType: Unit<QuantityType = Self>;

    /// Returns an iterator over the variants of `Self::UnitType`.
    fn iter_units<'a>() -> core::slice::Iter<'a, Self::UnitType> {
        Self::UnitType::iter()
    }

    /// Returns `Some(unit)` where `unit.symbol()` == `symbol`, or `None` if
    /// there is no such unit.
    fn unit_from_symbol(symbol: &str) -> Option<Self::UnitType> {
        for unit in Self::iter_units() {
            if unit.symbol() == symbol {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns `Some(unit)` where `unit.scale()` == `Some(amnt)`, or `None` if
    /// there is no such unit.
    fn unit_from_scale(amnt: AmountT) -> Option<Self::UnitType> {
        for unit in Self::iter_units() {
            if unit.scale() == Some(amnt) {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns a new instance of `Quantity<U>`.
    fn new(amount: AmountT, unit: Self::UnitType) -> Self;

    /// Returns the amount of `self`.
    fn amount(&self) -> AmountT;

    /// Returns the unit of `self`.
    fn unit(&self) -> Self::UnitType;

    /// Returns `Some(factor)` so that `factor` * `unit` == `self`, or `None` if
    /// such a factor can't be determined.
    fn equiv_amount(&self, unit: Self::UnitType) -> Option<AmountT> {
        if self.unit() == unit {
            Some(self.amount())
        } else {
            // TODO: add converters
            Some(self.unit().ratio(&unit)? * self.amount())
        }
    }

    /// Returns `Some(qty)` where `qty` == `self` and `qty.unit()` is `to_unit`,
    /// or `None` if the conversion can't be done.
    fn convert(&self, to_unit: Self::UnitType) -> Option<Self> {
        Some(Self::new(self.equiv_amount(to_unit)?, to_unit))
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit().symbol() {
            "" => write!(f, "{}", self.amount()),
            _ => write!(f, "{} {}", self.amount(), self.unit().symbol()),
        }
    }

    fn eq(&self, other: &Self) -> bool {
        if self.unit() == other.unit() {
            self.amount() == other.amount()
        } else {
            match other.equiv_amount(self.unit()) {
                None => false,
                Some(equiv_amount) => self.amount() == equiv_amount,
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit() == other.unit() {
            self.amount().partial_cmp(&other.amount())
        } else {
            match self.equiv_amount(other.unit()) {
                None => None,
                Some(equiv_amount) => equiv_amount.partial_cmp(&other.amount()),
            }
        }
    }

    fn add(self, rhs: Self) -> Self {
        match rhs.equiv_amount(self.unit()) {
            Some(equiv) => Self::new(self.amount() + equiv, self.unit()),
            None => panic!("Incompatible units!"),
        }
    }

    fn sub(self, rhs: Self) -> Self {
        match rhs.equiv_amount(self.unit()) {
            Some(equiv) => Self::new(self.amount() - equiv, self.unit()),
            None => panic!("Incompatible units!"),
        }
    }

    fn div(self, rhs: Self) -> Unitless {
        match rhs.equiv_amount(self.unit()) {
            Some(equiv) => Unitless::new(self.amount() / equiv, ONE),
            None => panic!("Incompatible units!"),
        }
    }
}
