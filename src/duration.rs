// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `Duration`.

use crate::prelude::*;

#[quantity]
#[ref_unit(Second, "s", NONE, "Reference unit of quantity `Duration`")]
#[unit(Nanosecond, "ns", NANO, 0.000000001, "0.000000001·s")]
#[unit(Microsecond, "µs", MICRO, 0.000001, "0.000001·s")]
#[unit(Millisecond, "ms", MILLI, 0.001, "0.001·s")]
#[unit(Minute, "min", 60, "60·s")]
#[unit(Hour, "h", 3600, "60·min")]
#[unit(Day, "d", 86400, "24·h")]
/// Duration: 'what a clock reads'
///
/// Reference unit: Second ('s')
///
/// Predefined units:
///
/// | Symbol | Name                  | Definition        | Equivalent in 's'   |
/// |--------|-----------------------|-------------------|---------------------|
/// | ns     | Nanosecond            | 0.000000001·s     | 0.000000001         |
/// | µs     | Microsecond           | 0.000001·s        | 0.000001            |
/// | ms     | Millisecond           | 0.001·s           | 0.001               |
/// | min    | Minute                | 60·s              | 60                  |
/// | h      | Hour                  | 60·min            | 3600                |
/// | d      | Day                   | 24·h              | 86400               |
pub struct Duration {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let amnt: AmountT = Amnt!(29.35);
        let d = amnt * MILLISECOND;
        assert_eq!(d.amount, amnt);
        assert_eq!(d.unit, MILLISECOND);
        #[cfg(feature = "std")]
        assert_eq!(d.to_string(), "29.35 ms");
    }
}
