// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#![doc = include_str ! ("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use quantities::prelude::*;

#[quantity]
#[ref_unit(Solar_Mass, "M☉", "Reference unit of quantity `Mass`")]
#[unit(Lunar_Mass, "M☾", 3.694329684197616e-8, "1/27068510·M☉")]
#[unit(Earth_Mass, "M🜨", 3.003489616124103e-6, "10000/3329460487·M☉")]
#[unit(Jupiter_Mass, "M♃", 9.547918983127074e-4, "1000000/1047348644·M☉")]
/// The quantity of matter in an astonomical body.
///
/// Reference unit: Solar Mass ('M☉')
///
/// Predefined units:
///
/// | Symbol | Name            | Definition            | Equivalent in 'M☉'   |
/// |--------|-----------------|-----------------------|----------------------|
/// | M☾     | Lunar Mass      | 1/27068510 M☉         | 3.694329684197616e-8 |
/// | M🜨     | Earth Mass      | 10000/3329460487 M☉   | 3.003489616124103e-6 |
/// | M♃     | Jupiter Mass    | 1000000/1047348644 M☉ | 9.547918983127074e-4 |
pub struct Mass {}

#[quantity]
#[ref_unit(
    Astronomical_Unit,
    "au",
    "Reference unit of quantity `Length` (= 149597870700·m)"
)]
#[unit(Kilometer, "km", 6.6845871222684464e-9, "1000·m")]
#[unit(Lightsecond, "ls", 0.002003988804100004, "299792458·m")]
#[unit(Lightyear, "ly", 63241.07708426629, "31557600·ls")]
#[unit(Parsec, "pc", 206264.80624709636, "648000/π·au")]
#[unit(Kilolightyear, "kly", 63241077.08426629, "1000·ly")]
#[unit(Kiloparsec, "kpc", 206264806.24709636, "1000·pc")]
#[unit(Megalightyear, "Mly", 63241077084.26629, "10⁶·ly")]
#[unit(Megaparsec, "Mpc", 206264806247.09636, "10⁶·pc")]
#[unit(Gigalightyear, "Gly", 63241077084266.29, "10⁹·ly")]
#[unit(Gigaparsec, "Gpc", 206264806247096.36, "10⁹·pc")]
/// The quantity of distance between two points in spacetime.
///
/// Reference unit: Astronomical Unit ('au')
///
/// Predefined units:
///
/// | Symbol | Name                    | Definition     | Equivalent in 'au'   |
/// |--------|-------------------------|----------------|----------------------|
/// | km     | Kilometer               | 1000·m         | 6.684587122268446e-9 |
/// | ls     | Lightsecond             | 299792458·m    | 0.002003988804100004 |
/// | ly     | Lightyear               | 31557600·ls    | 63241.07708426629    |
/// | pc     | Parsec                  | 648000/π·au    | 206264.80624709636   |
/// | kly    | Kilolightyear           | 1000·ly        | 63241077.08426629    |
/// | kpc    | Kiloparsec              | 1000·pc        | 206264806.24709636   |
/// | Mly    | Megalightyear           | 10⁶·ly         | 63241077084.26629    |
/// | Mpc    | Megaparsec              | 10⁶·pc         | 206264806247.09636   |
/// | Gly    | Gigalightyear           | 10⁹·ly         | 63241077084266.29    |
/// | Gpc    | Gigaparsec              | 10⁹·pc         | 206264806247096.36   |
pub struct Length {}

#[quantity]
#[ref_unit(Day, "d", "Reference unit of quantity `Duration` (= 24·h)")]
#[unit(Second, "s", 1.1574074074074073e-5, "SI reference unit")]
#[unit(Minute, "min", 0.0006944444444444445, "60·s")]
#[unit(Hour, "h", 0.041666666666666664, "60·min")]
#[unit(Sideral_Day, "dₛ", 0.9972685185185185, "a·d/(a + d)")]
#[unit(Julian_Year, "a", 365.25, "365.25·d")]
#[unit(Gregorian_Year, "yr", 365.2425, "365.2425·d")]
#[unit(
    Earth_Period,
    "T🜨",
    365.256363004,
    "Earth's Orbital Period (≈ 365.256363004·d)"
)]
/// Duration: 'what a clock reads'
///
/// Reference unit: Day ('d')
///
/// Predefined units:
///
/// | Symbol | Name                | Definition        | Equivalent in 'd'     |
/// |--------|---------------------|-------------------|-----------------------|
/// | s      | Second              | SI reference unit | 1.1574074074074073e-5 |
/// | min    | Minute              | 60·s              | 0.0006944444444444445 |
/// | h      | Hour                | 60·min            | 0.041666666666666664  |
/// | dₛ     | Siderial Day        | a·d/(a + d)       | 0.9972685185185185    |
/// | a      | Julian Year         | 365.25·d          | 365.25                |
/// | yr     | Gregorian Year      | 365.2425·d        | 365.2425              |
/// | T🜨     | Earth's Orbital Period | ≈ 365.256363004·d | 365.256363004      |
pub struct Duration {}

#[quantity(Length / Duration)]
#[ref_unit(
    Astronomical_Units_per_Day,
    "au/d",
    "Reference unit of quantity `Speed`"
)]
#[unit(Kilometer_per_Hour, "km/h", 1.604300909344427e-7, "km/h")]
#[unit(Meter_per_Second, "m/s", 5.775483273639937e-7, "SI reference unit")]
#[unit(Speed_of_Light, "c", 173.14463267424034, "ls/s")]
/// Magnitude of the change of an objects position per unit of time
///
/// Definition: Length/Duration
///
/// Reference unit: Astronomical Units per Day ('au/d')
///
/// Predefined units:
///
/// | Symbol | Name                 | Definition        | Equivalent in 'au/d' |
/// |--------|----------------------|-------------------|----------------------|
/// | km/h   | Kilometer per Hour   | km/h              | 1.604300909344427e-7 |
/// | m/s    | Meter per Second     | SI reference unit | 5.775483273639937e-7 |
/// | c      | Speed of Light       | ls/s              | 173.14463267424034   |
pub struct Speed {}
