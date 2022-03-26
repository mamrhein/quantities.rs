// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Force`.

use crate::acceleration::Acceleration;
use crate::mass::Mass;
use crate::prelude::*;

#[quantity(Mass * Acceleration)]
#[ref_unit(Newton, "N", NONE, "Reference unit of quantity `Force`")]
#[unit(Joule_per_Meter, "J/m", NONE, 1, "J/m")]
/// Influence that can accelerate an object with mass.
///
/// Definition: Mass·Acceleration = Mass·Length/Duration²
///
/// Reference unit: Newton ('N' = 'kg·m/s²')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition      | Equivalent in 'N'   |
/// |--------|-------------------------|-----------------|---------------------|
/// | J/m    | Joule per Meter         | J/m             | 1                   |
pub struct Force {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acceleration::METER_PER_SECOND_SQUARED;
    use crate::assert_almost_eq;
    use crate::mass::GRAM;

    #[test]
    fn test_mass_mul_acceleration() {
        let am = Amnt!(75.8);
        let m = am * GRAM;
        let aa: AmountT = Amnt!(9.4);
        let a = aa * METER_PER_SECOND_SQUARED;
        let f = m * a;
        assert_almost_eq!(f.amount(), aa * am / Amnt!(1000.));
        assert_eq!(f.unit(), NEWTON);
    }
}
