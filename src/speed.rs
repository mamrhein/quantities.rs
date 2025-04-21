// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Speed`.

use crate::{duration::Duration, length::Length, prelude::*};

#[quantity(Length / Duration)]
#[ref_unit(
    Meter_per_Second,
    "m/s",
    NONE,
    "Reference unit of quantity `Speed`"
)]
#[unit(Kilometer_per_Hour, "km/h", 0.2777777777777778, "km/h")]
#[unit(Miles_per_Hour, "mph", 0.44704, "mi/h")]
#[unit(Knot, "kn", 0.5144444444444445, "nmi/h")]
/// Magnitude of the change of an objects position per unit of time
///
/// Definition: Length/Duration
///
/// Reference unit: Meter per Second ('m/s')
///
/// Predefined units:
///
/// | Symbol | Name               | Definition        | Equivalent in 'm/s' |
/// |--------|--------------------|-------------------|---------------------|
/// | km/h   | Kilometer per Hour | km/h              | 5/18                |
/// | mph    | Miles per Hour     | mi/h              | 0.44704             |
/// | kn     | Knot               | nmi/h             | 0.5144444444444445  |
pub struct Speed {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_almost_eq,
        duration::{MINUTE, SECOND},
        length::{KILOMETER, METER, MILE},
    };

    #[test]
    fn test_speed() {
        assert_eq!(<Speed as HasRefUnit>::REF_UNIT, SpeedUnit::REF_UNIT);
        assert!(METER_PER_SECOND.is_ref_unit());
        let amnt: AmountT = Amnt!(235.4);
        let v = amnt * KILOMETER_PER_HOUR;
        assert_eq!(v.amount(), amnt);
        assert_eq!(v.unit(), KILOMETER_PER_HOUR);
        #[cfg(feature = "std")]
        assert_eq!(v.to_string(), "235.4 km/h");
    }

    #[test]
    fn test_length_div_duration() {
        let al: AmountT = Amnt!(35.1);
        let l = al * MILE;
        let at: AmountT = Amnt!(13.);
        let t = at * MINUTE;
        let v = l / t;
        assert_almost_eq!(v.amount(), al / at * Amnt!(0.44704) * Amnt!(60.));
        assert_eq!(v.unit(), METER_PER_SECOND);
    }

    #[test]
    fn test_speed_mul_duration() {
        let av: AmountT = Amnt!(2.94);
        let v = av * METER_PER_SECOND;
        let at = Amnt!(0.7);
        let t = at * MINUTE;
        let l = v * t;
        assert_almost_eq!(l.amount(), av * at * Amnt!(60.));
        assert_eq!(l.unit(), METER);
    }

    #[test]
    fn test_length_div_speed() {
        let al = Amnt!(0.7);
        let l = al * KILOMETER;
        let av: AmountT = Amnt!(2.94);
        let v = av * METER_PER_SECOND;
        let t = l / v;
        assert_almost_eq!(t.amount(), al * Amnt!(1000.) / av);
        assert_eq!(t.unit(), SECOND);
    }
}
