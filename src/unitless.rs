// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::{AmountT, Qty, Quantity, SIPrefix, Unit};
use core::ops::Mul;

/// Special "unitless" quantity.
///
/// An instances of this type is returned when an instance of a quantity is
/// divided by an instance of the same type of quantity.
pub type Unitless = Qty<One>;

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
    use crate::Amnt;

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
    }
}
