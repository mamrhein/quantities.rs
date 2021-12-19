// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SIPrefix {
    YOCTO,
    ZEPTO,
    ATTO,
    FEMTO,
    PICO,
    NANO,
    MICRO,
    MILLI,
    CENTI,
    DECI,
    NONE,
    DECA,
    HECTO,
    KILO,
    MEGA,
    GIGA,
    TERA,
    PETA,
    EXA,
    ZETTA,
    YOTTA,
}

impl SIPrefix {
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

    pub const fn exp(&self) -> i8 {
        match self {
            SIPrefix::YOCTO => -24,
            SIPrefix::ZEPTO => -21,
            SIPrefix::ATTO => -18,
            SIPrefix::FEMTO => -15,
            SIPrefix::PICO => -12,
            SIPrefix::NANO => -9,
            SIPrefix::MICRO => -6,
            SIPrefix::MILLI => -3,
            SIPrefix::CENTI => -2,
            SIPrefix::DECI => -1,
            SIPrefix::NONE => 0,
            SIPrefix::DECA => 1,
            SIPrefix::HECTO => 2,
            SIPrefix::KILO => 3,
            SIPrefix::MEGA => 6,
            SIPrefix::GIGA => 9,
            SIPrefix::TERA => 12,
            SIPrefix::PETA => 15,
            SIPrefix::EXA => 18,
            SIPrefix::ZETTA => 21,
            SIPrefix::YOTTA => 24,
        }
    }

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

    pub fn from_exp(exp: i8) -> Option<Self> {
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

pub const SI_PREFIXES: [SIPrefix; 21] = [
    SIPrefix::YOCTO,
    SIPrefix::ZEPTO,
    SIPrefix::ATTO,
    SIPrefix::FEMTO,
    SIPrefix::PICO,
    SIPrefix::NANO,
    SIPrefix::MICRO,
    SIPrefix::MILLI,
    SIPrefix::CENTI,
    SIPrefix::DECI,
    SIPrefix::NONE,
    SIPrefix::DECA,
    SIPrefix::HECTO,
    SIPrefix::KILO,
    SIPrefix::MEGA,
    SIPrefix::GIGA,
    SIPrefix::TERA,
    SIPrefix::PETA,
    SIPrefix::EXA,
    SIPrefix::ZETTA,
    SIPrefix::YOTTA,
];

#[cfg(test)]
mod tests {
    use super::*;

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
