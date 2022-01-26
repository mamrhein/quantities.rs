// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use ::qty_macros::EnumIter;

/// Enum of unit prefixes defined for the System of Units (SI).
///
/// These prefixes can be added to unit names to name multiples and submultiples
/// of the original unit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum SIPrefix {
    /// 10⁻²⁴
    YOCTO = -24,
    /// 10⁻²¹
    ZEPTO = -21,
    /// 10⁻¹⁸
    ATTO = -18,
    /// 10⁻¹⁵
    FEMTO = -15,
    /// 10⁻¹²
    PICO = -12,
    /// 10⁻⁹
    NANO = -9,
    /// 10⁻⁶
    MICRO = -6,
    /// 10⁻³
    MILLI = -3,
    /// 10⁻²
    CENTI = -2,
    /// 10⁻¹
    DECI = -1,
    /// 10⁰
    NONE = 0,
    /// 10¹
    DECA = 1,
    /// 10²
    HECTO = 2,
    /// 10³
    KILO = 3,
    /// 10⁶
    MEGA = 6,
    /// 10⁹
    GIGA = 9,
    /// 10¹²
    TERA = 12,
    /// 10¹⁵
    PETA = 15,
    /// 10¹⁸
    EXA = 18,
    /// 10²¹
    ZETTA = 21,
    /// 10²⁴
    YOTTA = 24,
}

impl SIPrefix {
    /// Returns the name of `self`.
    pub const fn name(&self) -> &'static str {
        match self {
            SIPrefix::YOCTO => "Yocto",
            SIPrefix::ZEPTO => "Zepto",
            SIPrefix::ATTO => "Atto",
            SIPrefix::FEMTO => "Femto",
            SIPrefix::PICO => "Pico",
            SIPrefix::NANO => "Nano",
            SIPrefix::MICRO => "Micro",
            SIPrefix::MILLI => "Milli",
            SIPrefix::CENTI => "Centi",
            SIPrefix::DECI => "Deci",
            SIPrefix::NONE => "",
            SIPrefix::DECA => "Deca",
            SIPrefix::HECTO => "Hecto",
            SIPrefix::KILO => "Kilo",
            SIPrefix::MEGA => "Mega",
            SIPrefix::GIGA => "Giga",
            SIPrefix::TERA => "Tera",
            SIPrefix::PETA => "Peta",
            SIPrefix::EXA => "Exa",
            SIPrefix::ZETTA => "Zetta",
            SIPrefix::YOTTA => "Yotta",
        }
    }

    /// Returns the abbreviation used to represent `self`.
    pub const fn abbr(&self) -> &'static str {
        match self {
            SIPrefix::YOCTO => "y",
            SIPrefix::ZEPTO => "z",
            SIPrefix::ATTO => "a",
            SIPrefix::FEMTO => "f",
            SIPrefix::PICO => "p",
            SIPrefix::NANO => "n",
            SIPrefix::MICRO => "µ",
            SIPrefix::MILLI => "m",
            SIPrefix::CENTI => "c",
            SIPrefix::DECI => "d",
            SIPrefix::NONE => "",
            SIPrefix::DECA => "da",
            SIPrefix::HECTO => "h",
            SIPrefix::KILO => "k",
            SIPrefix::MEGA => "M",
            SIPrefix::GIGA => "G",
            SIPrefix::TERA => "T",
            SIPrefix::PETA => "P",
            SIPrefix::EXA => "E",
            SIPrefix::ZETTA => "Z",
            SIPrefix::YOTTA => "Y",
        }
    }

    /// Returns the exponent of base 10 represented by `self`.
    #[inline(always)]
    pub const fn exp(&self) -> i8 {
        *self as i8
    }

    /// Returns the SI prefix with the abbreviation `abbr`, or `None` if there
    /// is no such SI prefix.
    pub fn from_abbr(abbr: &str) -> Option<Self> {
        match abbr {
            "y" => Some(SIPrefix::YOCTO),
            "z" => Some(SIPrefix::ZEPTO),
            "a" => Some(SIPrefix::ATTO),
            "f" => Some(SIPrefix::FEMTO),
            "p" => Some(SIPrefix::PICO),
            "n" => Some(SIPrefix::NANO),
            "µ" => Some(SIPrefix::MICRO),
            "m" => Some(SIPrefix::MILLI),
            "c" => Some(SIPrefix::CENTI),
            "d" => Some(SIPrefix::DECI),
            "" => Some(SIPrefix::NONE),
            "da" => Some(SIPrefix::DECA),
            "h" => Some(SIPrefix::HECTO),
            "k" => Some(SIPrefix::KILO),
            "M" => Some(SIPrefix::MEGA),
            "G" => Some(SIPrefix::GIGA),
            "T" => Some(SIPrefix::TERA),
            "P" => Some(SIPrefix::PETA),
            "E" => Some(SIPrefix::EXA),
            "Z" => Some(SIPrefix::ZETTA),
            "Y" => Some(SIPrefix::YOTTA),
            _ => None,
        }
    }

    /// Returns the SI prefix with the exponent `exp`, or `None` if there is no
    /// such SI prefix.
    pub const fn from_exp(exp: i8) -> Option<Self> {
        match exp {
            -24 => Some(SIPrefix::YOCTO),
            -21 => Some(SIPrefix::ZEPTO),
            -18 => Some(SIPrefix::ATTO),
            -15 => Some(SIPrefix::FEMTO),
            -12 => Some(SIPrefix::PICO),
            -9 => Some(SIPrefix::NANO),
            -6 => Some(SIPrefix::MICRO),
            -3 => Some(SIPrefix::MILLI),
            -2 => Some(SIPrefix::CENTI),
            -1 => Some(SIPrefix::DECI),
            0 => Some(SIPrefix::NONE),
            1 => Some(SIPrefix::DECA),
            2 => Some(SIPrefix::HECTO),
            3 => Some(SIPrefix::KILO),
            6 => Some(SIPrefix::MEGA),
            9 => Some(SIPrefix::GIGA),
            12 => Some(SIPrefix::TERA),
            15 => Some(SIPrefix::PETA),
            18 => Some(SIPrefix::EXA),
            21 => Some(SIPrefix::ZETTA),
            24 => Some(SIPrefix::YOTTA),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut it = SIPrefix::iter();
        assert_eq!(it.next(), Some(&SIPrefix::YOCTO));
        assert_eq!(it.next(), Some(&SIPrefix::ZEPTO));
        assert_eq!(it.next(), Some(&SIPrefix::ATTO));
        assert_eq!(it.next(), Some(&SIPrefix::FEMTO));
        assert_eq!(it.next(), Some(&SIPrefix::PICO));
        assert_eq!(it.next(), Some(&SIPrefix::NANO));
        assert_eq!(it.next(), Some(&SIPrefix::MICRO));
        assert_eq!(it.next(), Some(&SIPrefix::MILLI));
        assert_eq!(it.next(), Some(&SIPrefix::CENTI));
        assert_eq!(it.next(), Some(&SIPrefix::DECI));
        assert_eq!(it.next(), Some(&SIPrefix::NONE));
        assert_eq!(it.next(), Some(&SIPrefix::DECA));
        assert_eq!(it.next(), Some(&SIPrefix::HECTO));
        assert_eq!(it.next(), Some(&SIPrefix::KILO));
        assert_eq!(it.next(), Some(&SIPrefix::MEGA));
        assert_eq!(it.next(), Some(&SIPrefix::GIGA));
        assert_eq!(it.next(), Some(&SIPrefix::TERA));
        assert_eq!(it.next(), Some(&SIPrefix::PETA));
        assert_eq!(it.next(), Some(&SIPrefix::EXA));
        assert_eq!(it.next(), Some(&SIPrefix::ZETTA));
        assert_eq!(it.next(), Some(&SIPrefix::YOTTA));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_si_prefix_attrs() {
        let m = SIPrefix::MILLI;
        assert_eq!(m.name(), "Milli");
        assert_eq!(m.abbr(), "m");
        assert_eq!(m.exp(), -3);
    }

    #[test]
    fn test_from_abbr() {
        assert_eq!(SIPrefix::from_abbr("M").unwrap(), SIPrefix::MEGA);
        assert_eq!(SIPrefix::from_abbr("µ").unwrap(), SIPrefix::MICRO);
        assert_eq!(SIPrefix::from_abbr("Z").unwrap(), SIPrefix::ZETTA);
        assert!(SIPrefix::from_abbr("x").is_none());
    }

    #[test]
    fn test_from_exp() {
        assert_eq!(SIPrefix::from_exp(-18).unwrap(), SIPrefix::ATTO);
        assert_eq!(SIPrefix::from_exp(0).unwrap(), SIPrefix::NONE);
        assert_eq!(SIPrefix::from_exp(9).unwrap(), SIPrefix::GIGA);
        assert!(SIPrefix::from_exp(7).is_none());
    }
}
