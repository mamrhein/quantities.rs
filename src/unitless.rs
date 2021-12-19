// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::{
    define_qty, impl_mul_amnt_unit, AmountT, Qty, Quantity, SIPrefix, Unit,
};
use core::ops::Mul;

define_qty!(Unitless, NonUnit, NONUNIT, "", "");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amnt;

    #[test]
    fn test_unitless() {
        let amnt = Amnt!(17.4);
        let qty = Unitless::new(amnt, NONUNIT);
        assert_eq!(qty.amount(), amnt);
        let qty = amnt * NONUNIT;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), NONUNIT);
        let qty = NONUNIT * amnt;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), NONUNIT);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_unitless_to_string() {
        let amnt = Amnt!(184.09);
        let lit = amnt.to_string();
        let qty = Unitless::new(amnt, NONUNIT);
        assert_eq!(qty.to_string(), lit);
    }
}
