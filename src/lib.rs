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
pub use si_prefixes::SIPrefix;

#[cfg(feature = "fpdec")]
pub use amnt_dec::{AmountT, Dec, Decimal};
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "32"))]
pub use amnt_f32::AmountT;
#[cfg(all(not(feature = "fpdec"), target_pointer_width = "64"))]
pub use amnt_f64::AmountT;

pub mod prelude;
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
        match Self::REF_UNIT {
            Some(_) => Some(self.scale().unwrap() / other.scale().unwrap()),
            None => None,
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

    /// Returns a new instance of the type implementing `Quantity`.
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
            PartialOrd::partial_cmp(&self.amount(), &other.amount())
        } else {
            match self.equiv_amount(other.unit()) {
                None => None,
                Some(equiv_amount) => {
                    PartialOrd::partial_cmp(&equiv_amount, &other.amount())
                }
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

    fn div(self, rhs: Self) -> AmountT {
        match rhs.equiv_amount(self.unit()) {
            Some(equiv) => self.amount() / equiv,
            None => panic!("Incompatible units!"),
        }
    }
}

/// Trait for quantities having a reference unit
pub trait HasRefUnit: Quantity {
    /// Unit used as reference for scaling the units of `Self::UnitType`.
    const REF_UNIT: <Self as Quantity>::UnitType;

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
            .filter(|u| {
                u.scale().is_some()
                    && u.scale().unwrap() > first.scale().unwrap()
                    && u.scale().unwrap() <= amount
            })
            .last();
        match last {
            Some(unit) => Self::new(amount / unit.scale().unwrap(), *unit),
            None => Self::new(amount / first.scale().unwrap(), *first),
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
    const REF_UNIT: Option<Self> = Some(ONE);
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
    fn scale(&self) -> Option<AmountT> {
        Some(Amnt!(1.))
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
