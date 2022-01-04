// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod macro_attr_tests {
    use quantities::prelude::*;

    #[test]
    fn test_macro_attr_quantity() {
        #[quantity]
        #[ref_unit(Kilogram, "kg", KILO)]
        #[unit(Milligram, "mg", MILLI, 0.000001)]
        #[unit(Gram, "g", NONE, 0.001)]
        #[unit(Ounce, "oz", 0.028349523125)]
        #[unit(Pound, "lb", 0.45359237)]
        #[unit(Tonne, "t", MEGA, 1000.)]
        /// The quantity of matter in a physical body.
        struct Mass {}
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
