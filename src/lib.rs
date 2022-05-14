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

extern crate core;

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
#[cfg(feature = "volume")]
pub mod volume;

/// The abstract type of units used to define quantities.
pub trait Unit: Copy + Eq + PartialEq + Sized + Mul<AmountT> {
    /// Associated type of quantity
    type QuantityType: Quantity<UnitType = Self>;

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

    /// Returns the name of `self`.
    fn name(&self) -> &'static str;

    /// Returns the symbol used to represent `self`.
    fn symbol(&self) -> &'static str;

    /// Returns the SI prefix of `self`, or None is `self` is not a SI unit.
    fn si_prefix(&self) -> Option<SIPrefix>;

    // Returns `1 * self`
    fn as_qty(&self) -> Self::QuantityType {
        Self::QuantityType::new(AMNT_ONE, *self)
    }
}

/// Type of units being linear scaled in terms of a reference unit.
pub trait LinearScaledUnit: Unit {
    /// Unit used as reference for scaling the units.
    const REF_UNIT: Self;

    /// Returns `Some(unit)` where `unit.scale()` == `Some(amnt)`, or `None` if
    /// there is no such unit.
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

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.unit() == other.unit() && self.amount() == other.amount()
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit() == other.unit() {
            PartialOrd::partial_cmp(&self.amount(), &other.amount())
        } else {
            None
        }
    }

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

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit().symbol() {
            "" => write!(f, "{}", self.amount()),
            _ => write!(f, "{} {}", self.amount(), self.unit().symbol()),
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

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.amount() == other.equiv_amount(self.unit())
    }

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

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.amount() + rhs.equiv_amount(self.unit()), self.unit())
    }

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.amount() - rhs.equiv_amount(self.unit()), self.unit())
    }

    #[inline]
    fn div(self, rhs: Self) -> AmountT {
        self.amount() / rhs.equiv_amount(self.unit())
    }

    #[doc(hidden)]
    /// Returns a new instance of the type implementing `HasRefUnit`, equivalent
    /// to `amount * Self::REF_UNIT`, converted to the unit with the greatest
    /// scale less than or equal to `amount` or - if there is no such unit - to
    /// the unit with the smallest scale greater than `amount`, in any case
    /// taking only SI units into account if Self::REF_UNIT is a SI unit.
    fn _fit(amount: AmountT) -> Self {
        let take_all = Self::REF_UNIT.si_prefix().is_none();
        let mut it =
            Self::iter_units().filter(|u| take_all || u.si_prefix().is_some());
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
    fn name(&self) -> &'static str {
        "One"
    }
    fn symbol(&self) -> &'static str {
        ""
    }
    fn si_prefix(&self) -> Option<SIPrefix> {
        None
    }
}

impl LinearScaledUnit for One {
    const REF_UNIT: Self = ONE;
    fn scale(&self) -> AmountT {
        AMNT_ONE
    }
}

impl Mul<One> for AmountT {
    type Output = AmountT;
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
