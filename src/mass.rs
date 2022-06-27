// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Mass`.

use crate::prelude::*;

#[quantity]
#[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
#[unit(Milligram, "mg", MILLI, 0.000001, "0.001·g")]
#[unit(Carat, "ct", 0.0002, "0.2·g")]
#[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
#[unit(Ounce, "oz", 0.028349523125, "0.0625·lb")]
#[unit(Pound, "lb", 0.45359237, "0.45359237·kg")]
#[unit(Stone, "st", 6.35029318, "14·lb")]
#[unit(Tonne, "t", MEGA, 1000, "1000·kg")]
#[unit(Sol_Mass, "M☉", 1.988435e30, "1.988435·10^30·kg")]
#[unit(Earth_Mass, "M🜨", 5.97e24, "5.97·10^24·kg")]
#[unit(Luna_Mass, "M☾", 7.342e22, "7.342·10^22·kg")]
#[unit(Jupiter_Mass, "M♃", 1898e24, "1898·10^24·kg")]
/// The quantity of matter in a physical body.
///
/// Also used as measure of a physical body's resistance to acceleration.
///
/// Reference unit: Kilogram ('kg')
///
/// Predefined units:
///
/// | Symbol | Name                   | Definition        | Equivalent in 'kg' |
/// |--------|------------------------|-------------------|--------------------|
/// | mg     | Milligram              | 0.001·g           | 0.000001           |
/// | ct     | Carat                  | 0.2·g             | 0.0002             |
/// | g      | Gram                   | 0.001·kg          | 0.001              |
/// | oz     | Ounce                  | 0.0625·lb         | 0.028349523125     |
/// | lb     | Pound                  | 0.45359237·kg     | 0.45359237         |
/// | st     | Stone                  | 14·lb             | 6.35029318         |
/// | t      | Tonne                  | 1000·kg           | 1000               |
/// | M☉     | Sol Mass               | 1.988435e30·kg    | 1.988435e30        |
/// | M🜨     | Earth Mass             | 5.97e24·kg        | 5.97e24            |
/// | M☾     | Luna Mass              | 7.342e22·kg       | 7.342e22           |
/// | M♃     | Jupiter Mass           | 1898e24·kg        | 1898e24            |
pub struct Mass {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass() {
        assert_eq!(<Mass as HasRefUnit>::REF_UNIT, MassUnit::REF_UNIT);
        assert!(KILOGRAM.is_ref_unit());
        let amnt: AmountT = Amnt!(29.35);
        let m = amnt * KILOGRAM;
        assert_eq!(m.amount, amnt);
        assert_eq!(m.unit, KILOGRAM);
        #[cfg(feature = "std")]
        assert_eq!(m.to_string(), "29.35 kg");
    }

    #[test]
    fn test_0_9_1() {
        let ammount: AmountT = Amnt!(12.34);
        let m = ammount * EARTH_MASS;
        assert_eq!(m.amount, ammount);
        assert_eq!(m.unit, EARTH_MASS);
        #[cfg(feature = "std")]
        assert_eq!(m.to_string(), "12.34 M🜨");
        
    }
}
