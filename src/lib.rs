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
// activate some rustc lints
#![deny(non_ascii_idents)]
#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(unused)]
#![allow(dead_code)]
// activate some clippy lints
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::if_not_else)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::integer_division)]
#![warn(clippy::manual_assert)]
#![warn(clippy::match_same_arms)]
// #![warn(clippy::mismatching_type_param_order)] TODO: enable when got stable
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unicode_not_nfc)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_self)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::wildcard_imports)]

extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
};
use core::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[cfg(feature = "fpdec")]
pub use amnt_dec::{AmountT, Dec, Decimal, AMNT_ONE, AMNT_ZERO};
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "32"))]
pub use amnt_f32::{AmountT, AMNT_ONE, AMNT_ZERO};
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "64"))]
pub use amnt_f64::{AmountT, AMNT_ONE, AMNT_ZERO};
pub use converter::{ConversionTable, Converter};
pub use rate::Rate;
pub use si_prefixes::SIPrefix;

mod converter;
pub mod prelude;
mod rate;
mod si_prefixes;

#[cfg(feature = "fpdec")]
#[doc(hidden)]
pub mod amnt_dec;
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "32"))]
#[doc(hidden)]
pub mod amnt_f32;
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "64"))]
#[doc(hidden)]
pub mod amnt_f64;

#[cfg(feature = "acceleration")]
pub mod acceleration;
#[cfg(feature = "area")]
pub mod area;
#[cfg(feature = "datathroughput")]
pub mod datathroughput;
#[cfg(feature = "datavolume")]
pub mod datavolume;
#[cfg(feature = "duration")]
pub mod duration;
#[cfg(feature = "energy")]
pub mod energy;
#[cfg(feature = "force")]
pub mod force;
#[cfg(feature = "frequency")]
pub mod frequency;
#[cfg(feature = "length")]
pub mod length;
#[cfg(feature = "mass")]
pub mod mass;
#[cfg(feature = "power")]
pub mod power;
#[cfg(feature = "speed")]
pub mod speed;
#[cfg(feature = "temperature")]
pub mod temperature;
#[cfg(feature = "volume")]
pub mod volume;

/// The abstract type of units used to define quantities.
pub trait Unit:
    Copy + Eq + PartialEq + Sized + Mul<AmountT> + fmt::Display
{
    /// Associated type of quantity
    type QuantityType: Quantity<UnitType = Self>;

    /// Returns an iterator over the variants of `Self`.
    fn iter<'a>() -> core::slice::Iter<'a, Self>;

    /// Returns `Some(unit)` where `unit.symbol()` == `symbol`, or `None` if
    /// there is no such unit.
    #[must_use]
    fn from_symbol(symbol: &str) -> Option<Self> {
        for unit in Self::iter() {
            if unit.symbol() == symbol {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns the name of `self`.
    fn name(&self) -> String;

    /// Returns the symbol used to represent `self`.
    fn symbol(&self) -> String;

    /// Returns the SI prefix of `self`, or None is `self` is not a SI unit.
    fn si_prefix(&self) -> Option<SIPrefix>;

    /// Returns `1 * self`
    fn as_qty(&self) -> Self::QuantityType {
        Self::QuantityType::new(AMNT_ONE, *self)
    }

    /// Formats `self` using the given formatter.
    ///
    /// # Errors
    ///
    /// This function will only return an instance of `Error` returned from
    /// the formatter.
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.symbol(), form)
    }
}

/// Type of units being linear scaled in terms of a reference unit.
pub trait LinearScaledUnit: Unit {
    /// Unit used as reference for scaling the units.
    const REF_UNIT: Self;

    /// Returns `Some(unit)` where `unit.scale()` == `Some(amnt)`, or `None`
    /// if there is no such unit.
    #[must_use]
    fn from_scale(amnt: AmountT) -> Option<Self> {
        for unit in Self::iter() {
            if unit.scale() == amnt {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns `true` if `self` is the reference unit of its unit type.
    #[inline(always)]
    fn is_ref_unit(&self) -> bool {
        *self == Self::REF_UNIT
    }

    /// Returns `factor` so that `factor` * `Self::REFUNIT` == 1 * `self`.
    fn scale(&self) -> AmountT;

    /// Returns `factor` so that `factor` * `other` == 1 * `self`.
    #[inline(always)]
    fn ratio(&self, other: &Self) -> AmountT {
        self.scale() / other.scale()
    }
}

/// The abstract type of quantities.
pub trait Quantity: Copy + Sized + Mul<AmountT> {
    /// Associated type of unit
    type UnitType: Unit<QuantityType = Self>;

    /// Returns an iterator over the variants of `Self::UnitType`.
    fn iter_units<'a>() -> core::slice::Iter<'a, Self::UnitType> {
        Self::UnitType::iter()
    }

    /// Returns `Some(unit)` where `unit.symbol()` == `symbol`, or `None` if
    /// there is no such unit.
    #[must_use]
    fn unit_from_symbol(symbol: &str) -> Option<Self::UnitType> {
        for unit in Self::iter_units() {
            if unit.symbol() == symbol {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns a new instance of the type implementing `Quantity`.
    fn new(amount: AmountT, unit: Self::UnitType) -> Self;

    /// Returns the amount of `self`.
    fn amount(&self) -> AmountT;

    /// Returns the unit of `self`.
    fn unit(&self) -> Self::UnitType;

    /// Return `true` if `self` and `other` have the same unit and their
    /// amounts are equal, otherwise `false`.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.unit() == other.unit() && self.amount() == other.amount()
    }

    /// Returns the partial order of `self`s and `other`s amounts, if both
    /// have the same unit, otherwise `None`.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit() == other.unit() {
            PartialOrd::partial_cmp(&self.amount(), &other.amount())
        } else {
            None
        }
    }

    /// Returns the sum of `self` and `other`, if both have the same unit.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `other` have different units.
    fn add(self, rhs: Self) -> Self {
        if self.unit() == rhs.unit() {
            return Self::new(self.amount() + rhs.amount(), self.unit());
        }
        panic!(
            "Can't add '{}' and '{}'.",
            self.unit().symbol(),
            rhs.unit().symbol()
        );
    }

    /// Returns the difference between `self` and `other`, if both have the
    /// same unit.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `other` have different units.
    fn sub(self, rhs: Self) -> Self {
        if self.unit() == rhs.unit() {
            return Self::new(self.amount() - rhs.amount(), self.unit());
        }
        panic!(
            "Can't subtract '{}' and '{}'.",
            self.unit().symbol(),
            rhs.unit().symbol(),
        );
    }

    /// Returns the quotient `self` / `other`, if both have the same unit.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `other` have different units.
    fn div(self, rhs: Self) -> AmountT {
        if self.unit() == rhs.unit() {
            return self.amount() / rhs.amount();
        }
        panic!(
            "Can't divide '{}' and '{}'.",
            self.unit().symbol(),
            rhs.unit().symbol()
        );
    }

    /// Formats `self` using the given formatter.
    ///
    /// # Errors
    ///
    /// This function will only return an instance of `Error` returned from
    /// the formatter.
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unit().symbol().is_empty() {
            fmt::Display::fmt(&self.amount(), form)
        } else {
            let tmp: String;
            let amnt_non_neg = self.amount() >= AMNT_ZERO;
            #[cfg(feature = "fpdec")]
            let abs_amnt = self.amount().abs();
            #[cfg(not(feature = "fpdec"))]
            let abs_amnt = if amnt_non_neg {
                self.amount()
            } else {
                -self.amount()
            };
            if let Some(prec) = form.precision() {
                tmp = format!("{:.*} {}", prec, abs_amnt, self.unit());
            } else {
                tmp = format!("{} {}", abs_amnt, self.unit());
            }
            form.pad_integral(amnt_non_neg, "", &tmp)
        }
    }
}

/// Trait for quantities having a reference unit
pub trait HasRefUnit: Quantity + Add<Self> + Sub<Self> + Div<Self>
where
    <Self as Quantity>::UnitType: LinearScaledUnit,
{
    /// Unit used as reference for scaling the units of `Self::UnitType`.
    const REF_UNIT: <Self as Quantity>::UnitType;

    /// Returns `Some(unit)` where `unit.scale()` == `amnt`, or `None` if
    /// there is no such unit.
    #[must_use]
    fn unit_from_scale(amnt: AmountT) -> Option<Self::UnitType> {
        for unit in Self::iter_units() {
            if unit.scale() == amnt {
                return Some(*unit);
            }
        }
        None
    }

    /// Returns `factor` so that `factor` * `unit` == `self`.
    #[inline(always)]
    fn equiv_amount(&self, unit: Self::UnitType) -> AmountT {
        if self.unit() == unit {
            self.amount()
        } else {
            self.unit().ratio(&unit) * self.amount()
        }
    }

    /// Returns `qty` where `qty` == `self` and `qty.unit()` is `to_unit`.
    fn convert(&self, to_unit: Self::UnitType) -> Self {
        Self::new(self.equiv_amount(to_unit), to_unit)
    }

    /// Returns true, if `self` and `other` have equivalent amounts, otherwise
    /// `false`.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.equiv_amount(self.unit())
    }

    /// Returns the partial order of `self`s amount and `other`s eqivalent
    /// amount in `self`s unit.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit() == other.unit() {
            PartialOrd::partial_cmp(&self.amount(), &other.amount())
        } else {
            PartialOrd::partial_cmp(
                &self.amount(),
                &other.equiv_amount(self.unit()),
            )
        }
    }

    /// Returns the sum of `self` and `other`
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.amount() + rhs.equiv_amount(self.unit()), self.unit())
    }

    /// Returns the difference between `self` and `other`
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.amount() - rhs.equiv_amount(self.unit()), self.unit())
    }

    /// Returns the quotient `self` / `other`
    #[inline]
    fn div(self, rhs: Self) -> AmountT {
        self.amount() / rhs.equiv_amount(self.unit())
    }

    #[doc(hidden)]
    /// Returns a new instance of the type implementing `HasRefUnit`,
    /// equivalent to `amount * Self::REF_UNIT`, converted to the unit
    /// with the greatest scale less than or equal to `amount` or - if
    /// there is no such unit - to the unit with the smallest scale
    /// greater than `amount`, in any case taking only SI units into
    /// account if Self::REF_UNIT is a SI unit.
    #[must_use]
    fn _fit(amount: AmountT) -> Self {
        let take_all = Self::REF_UNIT.si_prefix().is_none();
        let mut it = Self::iter_units()
            .filter(|u| take_all || u.si_prefix().is_some());
        // `it` returns atleast the reference unit, so its safe to unwrap here
        let first = it.next().unwrap();
        let last = it
            .filter(|u| u.scale() > first.scale() && u.scale() <= amount)
            .last();
        match last {
            Some(unit) => Self::new(amount / unit.scale(), *unit),
            None => Self::new(amount / first.scale(), *first),
        }
    }
}

/// The "unit" of the "unitless" quantity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum One {
    /// Special singleton used as "unit" for the "unitless" quantity.
    One,
}

impl One {
    const VARIANTS: [Self; 1] = [ONE];
}

/// Special singleton used as "unit" for the "unitless" quantity.
pub const ONE: One = One::One;

impl Unit for One {
    type QuantityType = AmountT;
    fn iter<'a>() -> core::slice::Iter<'a, Self> {
        Self::VARIANTS.iter()
    }
    fn name(&self) -> String {
        "One".to_string()
    }
    fn symbol(&self) -> String {
        "".to_string()
    }
    fn si_prefix(&self) -> Option<SIPrefix> {
        None
    }
}

impl fmt::Display for One {
    #[inline(always)]
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Unit>::fmt(self, form)
    }
}

impl LinearScaledUnit for One {
    const REF_UNIT: Self = ONE;
    fn scale(&self) -> AmountT {
        AMNT_ONE
    }
}

impl Mul<One> for AmountT {
    type Output = Self;
    #[inline(always)]
    fn mul(self, _rhs: One) -> Self::Output {
        self
    }
}

impl Mul<AmountT> for One {
    type Output = AmountT;
    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        rhs
    }
}

impl Quantity for AmountT {
    type UnitType = One;

    #[inline(always)]
    fn new(amount: AmountT, _unit: Self::UnitType) -> Self {
        amount
    }

    #[inline(always)]
    fn amount(&self) -> AmountT {
        *self
    }

    #[inline(always)]
    fn unit(&self) -> Self::UnitType {
        ONE
    }
}

impl HasRefUnit for AmountT {
    const REF_UNIT: One = ONE;

    #[inline(always)]
    fn _fit(amount: AmountT) -> Self {
        amount
    }
}
