// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Area`.

use crate::length::Length;
use crate::prelude::*;

#[quantity(Length * Length)]
#[ref_unit(Square_Meter, "m²", NONE, "Reference unit of quantity `Area`")]
#[unit(Square_Millimeter, "mm²", MICRO, 0.000001, "mm²")]
#[unit(Square_Centimeter, "cm²", 0.0001, "cm²")]
#[unit(Square_Inch, "in²", 0.00064516, "in²")]
#[unit(Square_Decimeter, "dm²", CENTI, 0.01, "dm²")]
#[unit(Square_Foot, "ft²", 0.09290304, "ft²")]
#[unit(Square_Yard, "yd²", 0.83612736, "yd²")]
#[unit(Are, "a", HECTO, 100, "100·m²")]
#[unit(Acre, "ac", 4046.8564224, "4840·yd²")]
#[unit(Hectare, "ha", 10000, "100·a")]
#[unit(Square_Kilometer, "km²", MEGA, 1000000, "km²")]
#[unit(Square_Mile, "mi²", 2589988.110336, "mi²")]
/// The quantity expressing the extent of a two-dimensional region.
///
/// Definition: Length²
///
/// Reference unit: Square Meter ('m²')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition        | Equivalent in 'm²'  |
/// |--------|-------------------------|-------------------|---------------------|
/// | mm²    | Square Millimeter       | mm²               | 0.000001            |
/// | cm²    | Square Centimeter       | cm²               | 0.0001              |
/// | in²    | Square Inch             | in²               | 0.00064516          |
/// | dm²    | Square Decimeter        | dm²               | 0.01                |
/// | ft²    | Square Foot             | ft²               | 0.09290304          |
/// | yd²    | Square Yard             | yd²               | 0.83612736          |
/// | a      | Are                     | 100·m²            | 100                 |
/// | ac     | Acre                    | 4840·yd²          | 4046.8564224        |
/// | ha     | Hectare                 | 100·a             | 10000               |
/// | km²    | Square Kilometer        | km²               | 1000000             |
/// | mi²    | Square Mile             | mi²               | 2589988.110336      |
pub struct Area {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_almost_eq;
    use crate::length::{CENTIMETER, KILOMETER, METER};

    #[test]
    fn test_area() {
        assert_eq!(<Area as HasRefUnit>::REF_UNIT, AreaUnit::REF_UNIT);
        assert!(SQUARE_METER.is_ref_unit());
        let amnt: AmountT = Amnt!(29.35);
        let l = amnt * SQUARE_CENTIMETER;
        assert_eq!(l.amount(), amnt);
        assert_eq!(l.unit(), SQUARE_CENTIMETER);
        #[cfg(feature = "std")]
        assert_eq!(l.to_string(), "29.35 cm²");
    }

    #[test]
    fn test_length_mul_length() {
        let amnt: AmountT = Amnt!(29.3);
        let l = amnt * CENTIMETER;
        let a = l * l;
        assert_almost_eq!(a.amount(), amnt * amnt);
        assert_eq!(a.unit(), SQUARE_CENTIMETER);
        let w = Amnt!(2.) * KILOMETER;
        let h = amnt * CENTIMETER;
        let a = w * h;
        assert_almost_eq!(a.amount(), Amnt!(0.2) * amnt);
        assert_eq!(a.unit(), ARE);
    }

    #[test]
    fn test_aera_div_length() {
        let amnt: AmountT = Amnt!(29.4);
        let a = amnt * HECTARE;
        let w = Amnt!(0.7) * KILOMETER;
        let h = a / w;
        assert_almost_eq!(h.amount(), Amnt!(420.));
        assert_eq!(h.unit(), METER);
    }
}
