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
#[ref_unit(Kilogram, "kg", KILO)]
#[unit(Milligram, "mg", MILLI, 0.000001)]
#[unit(Carat, "ct", 0.0002)]
#[unit(Gram, "g", NONE, 0.001)]
#[unit(Ounce, "oz", 0.028349523125)]
#[unit(Pound, "lb", 0.45359237)]
#[unit(Tonne, "t", MEGA, 1000.)]
/// The quantity of matter in a physical body.
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
pub struct Mass {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass() {
        let m = Amnt!(29.35) * KILOGRAM;
        assert_eq!(m.to_string(), "29.35 kg");
    }
}
