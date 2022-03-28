// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Acceleration`.

use crate::duration::Duration;
use crate::prelude::*;
use crate::speed::Speed;

#[quantity(Speed / Duration)]
#[ref_unit(
    Meter_per_Second_squared,
    "m/s²",
    NONE,
    "Reference unit of quantity `Acceleration`"
)]
#[unit(Yards_per_Second_squared, "yd/s²", 0.9144, "yd/s²")]
/// Rate of change of an objects speed with respect to time.
///
/// Definition: Speed/Duration = Length/Duration²
///
/// Reference unit: Meter per Second squared ('m/s²')
///
/// Predefined units:
///
/// | Symbol | Name                     | Definition    | Equivalent in 'm/s²' |
/// |--------|--------------------------|---------------|----------------------|
/// | yd/s²  | Yards per Second squared | yd/s²         | 0.9144               |
pub struct Acceleration {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_almost_eq;
    use crate::duration::MILLISECOND;
    use crate::speed::METER_PER_SECOND;

    #[test]
    fn test_speed_div_duration() {
        let av: AmountT = Amnt!(2.94);
        let v = av * METER_PER_SECOND;
        let at = Amnt!(7.);
        let t = at * MILLISECOND;
        let a = v / t;
        assert_almost_eq!(a.amount(), av / at * Amnt!(1000.));
        assert_eq!(a.unit(), METER_PER_SECOND_SQUARED);
    }
}
