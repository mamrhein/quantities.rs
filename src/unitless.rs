// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::prelude::*;

#[quantity]
#[unit(NonUnit, "")]
/// Special "unitless" quantity.
///
/// An instances of this type is returned when an instance of a Quantity is
/// divided by an instance of the same type of Quantity.
pub struct Unitless {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amnt;

    #[test]
    fn test_unitless() {
        let amnt = Amnt!(17.4);
        let qty = Unitless::new(amnt, NON_UNIT);
        assert_eq!(qty.amount(), amnt);
        let qty = amnt * NON_UNIT;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), NON_UNIT);
        let qty = NON_UNIT * amnt;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), NON_UNIT);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_unitless_to_string() {
        let amnt = Amnt!(184.09);
        let lit = amnt.to_string();
        let qty = Unitless::new(amnt, NON_UNIT);
        assert_eq!(qty.to_string(), lit);
    }
}
