// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Power`.

use crate::{duration::Duration, energy::Energy, prelude::*};

#[quantity(Energy / Duration)]
#[ref_unit(Watt, "W", NONE, "Reference unit of quantity `Power`")]
#[unit(Milliwatt, "mW", MILLI, 0.001, "0.001·W")]
#[unit(Kilowatt, "kW", KILO, 1000, "1000·W")]
#[unit(Megawatt, "MW", MEGA, 1000000, "1000000·W")]
#[unit(Gigawatt, "GW", GIGA, 1000000000, "1000000000·W")]
#[unit(Terawatt, "TW", TERA, 1000000000000., "1000000000000·W")]
/// Energy transferred or converted per unit of time
///
/// Definition: Energy/Duration
///
/// Reference unit: Watt ('W' = 'J/s' = 'kg·m²/s³')
///
/// Predefined units:
///
/// | Symbol | Name                  | Definition        | Equivalent in 'W'   |
/// |--------|-----------------------|-------------------|---------------------|
/// | mW     | Milliwatt             | 0.001·W           | 0.001               |
/// | kW     | Kilowatt              | 1000·W            | 1000                |
/// | MW     | Megawatt              | 1000000·W         | 1000000             |
/// | GW     | Gigawatt              | 1000000000·W      | 1000000000          |
/// | TW     | Terawatt              | 1000000000000·W   | 1000000000000       |
pub struct Power {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_almost_eq, duration::MINUTE, energy::KILOWATT_HOUR};

    #[test]
    fn test_energy_div_duration() {
        let ae: AmountT = Amnt!(90.3);
        let e = ae * KILOWATT_HOUR;
        let at: AmountT = Amnt!(30.);
        let t = at * MINUTE;
        let p = e / t;
        assert_almost_eq!(p.amount(), ae / at * Amnt!(60.));
        assert_eq!(p.unit(), KILOWATT);
    }
}
