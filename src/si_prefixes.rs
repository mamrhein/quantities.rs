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
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::YOCTO => "Yocto",
            Self::ZEPTO => "Zepto",
            Self::ATTO => "Atto",
            Self::FEMTO => "Femto",
            Self::PICO => "Pico",
            Self::NANO => "Nano",
            Self::MICRO => "Micro",
            Self::MILLI => "Milli",
            Self::CENTI => "Centi",
            Self::DECI => "Deci",
            Self::NONE => "",
            Self::DECA => "Deca",
            Self::HECTO => "Hecto",
            Self::KILO => "Kilo",
            Self::MEGA => "Mega",
            Self::GIGA => "Giga",
            Self::TERA => "Tera",
            Self::PETA => "Peta",
            Self::EXA => "Exa",
            Self::ZETTA => "Zetta",
            Self::YOTTA => "Yotta",
        }
    }

    /// Returns the abbreviation used to represent `self`.
    #[must_use]
    pub const fn abbr(&self) -> &'static str {
        match self {
            Self::YOCTO => "y",
            Self::ZEPTO => "z",
            Self::ATTO => "a",
            Self::FEMTO => "f",
            Self::PICO => "p",
            Self::NANO => "n",
            Self::MICRO => "µ",
            Self::MILLI => "m",
            Self::CENTI => "c",
            Self::DECI => "d",
            Self::NONE => "",
            Self::DECA => "da",
            Self::HECTO => "h",
            Self::KILO => "k",
            Self::MEGA => "M",
            Self::GIGA => "G",
            Self::TERA => "T",
            Self::PETA => "P",
            Self::EXA => "E",
            Self::ZETTA => "Z",
            Self::YOTTA => "Y",
        }
    }

    /// Returns the exponent of base 10 represented by `self`.
    #[inline(always)]
    #[must_use]
    pub const fn exp(&self) -> i8 {
        *self as i8
    }

    /// Returns the SI prefix with the abbreviation `abbr`, or `None` if there
    /// is no such SI prefix.
    #[must_use]
    pub fn from_abbr(abbr: &str) -> Option<Self> {
        match abbr {
            "y" => Some(Self::YOCTO),
            "z" => Some(Self::ZEPTO),
            "a" => Some(Self::ATTO),
            "f" => Some(Self::FEMTO),
            "p" => Some(Self::PICO),
            "n" => Some(Self::NANO),
            "µ" => Some(Self::MICRO),
            "m" => Some(Self::MILLI),
            "c" => Some(Self::CENTI),
            "d" => Some(Self::DECI),
            "" => Some(Self::NONE),
            "da" => Some(Self::DECA),
            "h" => Some(Self::HECTO),
            "k" => Some(Self::KILO),
            "M" => Some(Self::MEGA),
            "G" => Some(Self::GIGA),
            "T" => Some(Self::TERA),
            "P" => Some(Self::PETA),
            "E" => Some(Self::EXA),
            "Z" => Some(Self::ZETTA),
            "Y" => Some(Self::YOTTA),
            _ => None,
        }
    }

    /// Returns the SI prefix with the exponent `exp`, or `None` if there is no
    /// such SI prefix.
    #[must_use]
    pub const fn from_exp(exp: i8) -> Option<Self> {
        match exp {
            -24 => Some(Self::YOCTO),
            -21 => Some(Self::ZEPTO),
            -18 => Some(Self::ATTO),
            -15 => Some(Self::FEMTO),
            -12 => Some(Self::PICO),
            -9 => Some(Self::NANO),
            -6 => Some(Self::MICRO),
            -3 => Some(Self::MILLI),
            -2 => Some(Self::CENTI),
            -1 => Some(Self::DECI),
            0 => Some(Self::NONE),
            1 => Some(Self::DECA),
            2 => Some(Self::HECTO),
            3 => Some(Self::KILO),
            6 => Some(Self::MEGA),
            9 => Some(Self::GIGA),
            12 => Some(Self::TERA),
            15 => Some(Self::PETA),
            18 => Some(Self::EXA),
            21 => Some(Self::ZETTA),
            24 => Some(Self::YOTTA),
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
