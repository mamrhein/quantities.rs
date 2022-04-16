// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `DataThroughput`.

use crate::{datavolume::DataVolume, duration::Duration, prelude::*};

#[quantity(DataVolume / Duration)]
#[ref_unit(
    Byte_per_Second,
    "B/s",
    NONE,
    "Reference unit of quantity `DataThroughput`"
)]
#[unit(Bit_per_Second, "b/s", 0.125, "b/s")]
#[unit(Kilobit_per_Second, "kb/s", 125, "1000·b/s")]
#[unit(Kibibit_per_Second, "Kib/s", 128, "1024·b/s")]
#[unit(Kilobyte_per_Second, "kB/s", KILO, 1000, "1000·B/s")]
#[unit(Kibibyte_per_Second, "KiB/s", 1024, "1024·B/s")]
#[unit(Megabit_per_Second, "Mb/s", 125000, "1000000·b/s")]
#[unit(Mebibit_per_Second, "Mib/s", 131072, "1048576·b/s")]
#[unit(Megabyte_per_Second, "MB/s", MEGA, 1000000, "1000000·B/s")]
#[unit(Mebibyte_per_Second, "MiB/s", 1048576, "1048576·B/s")]
#[unit(Gigabit_per_Second, "Gb/s", 125000000, "1000000000·b/s")]
#[unit(Gibibit_per_Second, "Gib/s", 134217728, "1073741824·b/s")]
#[unit(Gigabyte_per_Second, "GB/s", GIGA, 1000000000, "1000000000·B/s")]
#[unit(Gibibyte_per_Second, "GiB/s", 1073741824, "1073741824·B/s")]
#[unit(Terabit_per_Second, "Tb/s", 125000000000., "1000000000000·b/s")]
#[unit(Tebibit_per_Second, "Tib/s", 137438953472., "1099511627776·b/s")]
#[unit(Terabyte_per_Second, "TB/s", TERA, 1000000000000., "1000000000000·B/s")]
#[unit(Tebibyte_per_Second, "TiB/s", 1099511627776., "1099511627776·B/s")]
/// Volume of data transferred per unit of time
///
/// Definition: DataVolume/Duration
///
/// Reference unit: Byte per Second ('B/s')
///
/// Predefined units:
///
/// | Symbol | Name                  | Definition        | Equivalent in 'B/s' |
/// |--------|-----------------------|-------------------|---------------------|
/// | b/s    | Bit per Second        | b/s               | 0.125               |
/// | kb/s   | Kilobit per Second    | 1000·b/s          | 125                 |
/// | Kib/s  | Kibibit per Second    | 1024·b/s          | 128                 |
/// | kB/s   | Kilobyte per Second   | 1000·B/s          | 1000                |
/// | KiB/s  | Kibibyte per Second   | 1024·B/s          | 1024                |
/// | Mb/s   | Megabit per Second    | 1000000·b/s       | 125000              |
/// | Mib/s  | Mebibit per Second    | 1048576·b/s       | 131072              |
/// | MB/s   | Megabyte per Second   | 1000000·B/s       | 1000000             |
/// | MiB/s  | Mebibyte per Second   | 1048576·B/s       | 1048576             |
/// | Gb/s   | Gigabit per Second    | 1000000000·b/s    | 125000000           |
/// | Gib/s  | Gibibit per Second    | 1073741824·b/s    | 134217728           |
/// | GB/s   | Gigabyte per Second   | 1000000000·B/s    | 1000000000          |
/// | GiB/s  | Gibibyte per Second   | 1073741824·B/s    | 1073741824          |
/// | Tb/s   | Terabit per Second    | 1000000000000·b/s | 125000000000        |
/// | Tib/s  | Tebibit per Second    | 1099511627776·b/s | 137438953472        |
/// | TB/s   | Terabyte per Second   | 1000000000000·B/s | 1000000000000       |
/// | TiB/s  | Tebibyte per Second   | 1099511627776·B/s | 1099511627776       |
pub struct DataThroughput {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_almost_eq, datavolume::MEBIBYTE, duration::MILLISECOND,
    };

    #[test]
    fn test_datavolume_div_duration() {
        let ad: AmountT = Amnt!(837.5);
        let d = ad * MEBIBYTE;
        let at = Amnt!(2.5);
        let t = at * MILLISECOND;
        let r = d / t;
        assert_almost_eq!(r.amount(), ad / at * Amnt!(1.048576));
        assert_eq!(r.unit(), GIGABYTE_PER_SECOND);
    }
}
