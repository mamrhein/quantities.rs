// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::prelude::*;

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
    type QuantityType = Unitless;
    const REF_UNIT: Option<Self> = None;
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
        None
    }
}

/// Special "unitless" quantity.
///
/// An instances of this type is returned when an instance of a quantity is
/// divided by an instance of the same type of quantity.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Unitless {
    amount: AmountT,
}

impl Quantity for Unitless {
    type UnitType = One;

    #[inline(always)]
    fn new(amount: AmountT, _unit: Self::UnitType) -> Self {
        Self { amount }
    }

    #[inline(always)]
    fn amount(&self) -> AmountT {
        self.amount
    }

    #[inline(always)]
    fn unit(&self) -> Self::UnitType {
        ONE
    }
}

impl fmt::Display for Unitless {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Quantity::fmt(self, f)
    }
}

impl Add<Self> for Unitless {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount() + rhs.amount(),
        }
    }
}

impl Sub<Self> for Unitless {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount() - rhs.amount(),
        }
    }
}

impl Div<Self> for Unitless {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            amount: self.amount() / rhs.amount(),
        }
    }
}

impl Mul<AmountT> for Unitless {
    type Output = Self;

    fn mul(self, rhs: AmountT) -> Self::Output {
        Self::Output {
            amount: self.amount() * rhs,
        }
    }
}

impl Mul<Unitless> for AmountT {
    type Output = Unitless;

    fn mul(self, rhs: Unitless) -> Self::Output {
        Self::Output {
            amount: self * rhs.amount(),
        }
    }
}

impl Mul<One> for AmountT {
    type Output = Unitless;
    #[inline(always)]
    fn mul(self, rhs: One) -> Self::Output {
        Unitless::new(self, rhs)
    }
}

impl Mul<AmountT> for One {
    type Output = Unitless;
    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Unitless::new(rhs, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unitless() {
        let amnt = Amnt!(17.4);
        let qty = Unitless::new(amnt, ONE);
        assert_eq!(qty.amount(), amnt);
        let qty = amnt * ONE;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), ONE);
        let qty = ONE * amnt;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), ONE);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_unitless_to_string() {
        let amnt = Amnt!(184.09);
        let lit = amnt.to_string();
        let qty = Unitless::new(amnt, ONE);
        assert_eq!(qty.to_string(), lit);
        assert_eq!(format!("{}", qty), lit);
    }
}
