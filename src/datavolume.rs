// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of basic quantity `DataVolume`.

use crate::prelude::*;

#[quantity]
#[ref_unit(Byte, "B", NONE, "Reference unit of quantity `DataVolume`")]
#[unit(Bit, "b", 0.125, "0.125·B")]
#[unit(Kilobit, "kb", 125, "1000·b")]
#[unit(Kibibit, "Kib", 128, "1024·b")]
#[unit(Kilobyte, "kB", KILO, 1000, "1000·B")]
#[unit(Kibibyte, "KiB", 1024, "1024·B")]
#[unit(Megabit, "Mb", 125000, "1000000·b")]
#[unit(Mebibit, "Mib", 131072, "1048576·b")]
#[unit(Megabyte, "MB", MEGA, 1000000, "1000000·B")]
#[unit(Mebibyte, "MiB", 1048576, "1048576·B")]
#[unit(Gigabit, "Gb", 125000000, "1000000000·b")]
#[unit(Gibibit, "Gib", 134217728, "1073741824·b")]
#[unit(Gigabyte, "GB", GIGA, 1000000000, "1000000000·B")]
#[unit(Gibibyte, "GiB", 1073741824, "1073741824·B")]
#[unit(Terabit, "Tb", 125000000000., "1000000000000·b")]
#[unit(Tebibit, "Tib", 137438953472., "1099511627776·b")]
#[unit(Terabyte, "TB", TERA, 1000000000000., "1000000000000·B")]
#[unit(Tebibyte, "TiB", 1099511627776., "1099511627776·B")]
/// DataVolume according to IEEE 1541-2002
///
/// Reference unit: Byte ('B')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition        | Equivalent in 'B'   |
/// |--------|-------------------------|-------------------|---------------------|
/// | b      | Bit                     | 0.125·B           | 0.125               |
/// | kb     | Kilobit                 | 1000·b            | 125                 |
/// | Kib    | Kibibit                 | 1024·b            | 128                 |
/// | kB     | Kilobyte                | 1000·B            | 1000                |
/// | KiB    | Kibibyte                | 1024·B            | 1024                |
/// | Mb     | Megabit                 | 1000000·b         | 125000              |
/// | Mib    | Mebibit                 | 1048576·b         | 131072              |
/// | MB     | Megabyte                | 1000000·B         | 1000000             |
/// | MiB    | Mebibyte                | 1048576·B         | 1048576             |
/// | Gb     | Gigabit                 | 1000000000·b      | 125000000           |
/// | Gib    | Gibibit                 | 1073741824·b      | 134217728           |
/// | GB     | Gigabyte                | 1000000000·B      | 1000000000          |
/// | GiB    | Gibibyte                | 1073741824·B      | 1073741824          |
/// | Tb     | Terabit                 | 1000000000000·b   | 125000000000        |
/// | Tib    | Tebibit                 | 1099511627776·b   | 137438953472        |
/// | TB     | Terabyte                | 1000000000000·B   | 1000000000000       |
/// | TiB    | Tebibyte                | 1099511627776·B   | 1099511627776       |
pub struct DataVolume {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datavolume() {
        let amnt: AmountT = Amnt!(375);
        let d = amnt * GIBIBYTE;
        assert_eq!(d.amount, amnt);
        assert_eq!(d.unit, GIBIBYTE);
        #[cfg(feature = "std")]
        assert_eq!(d.to_string(), "375 GiB");
        let d = d.convert(TERABYTE).unwrap();
        assert_eq!(d.unit, TERABYTE);
        assert_eq!(d.amount, Amnt!(0.402653184));
        let d = d.convert(KIBIBYTE).unwrap();
        assert_eq!(d.unit, KIBIBYTE);
        assert_eq!(d.amount, Amnt!(393216000));
    }
}
