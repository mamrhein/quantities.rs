// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod rate_tests {
    use core::fmt;

    use quantities::{assert_almost_eq, prelude::*};

    #[quantity]
    #[ref_unit(Flop, "f")]
    #[unit(Kiloflop, "kf", 1000., "1000·f")]
    #[unit(Centiflop, "cf", 0.01, "0.01·f")]
    struct Foo {}

    #[quantity]
    #[ref_unit(Emil, "e")]
    #[unit(Milliemil, "me", 0.001, "0.001·e")]
    #[unit(Microemil, "µe", 0.000001, "0.000001·e")]
    #[unit(Kiloemil, "ke", 1000., "1000·e")]
    struct Bar {}

    #[test]
    fn test_rate() {
        let r =
            Rate::<Foo, Bar>::new(Amnt!(329.4), FLOP, Amnt!(100), MILLIEMIL);
        assert_eq!(r.term_amount(), Amnt!(329.4));
        assert_eq!(r.term_unit(), FLOP);
        assert_eq!(r.per_unit_multiple(), Amnt!(100));
        assert_eq!(r.per_unit(), MILLIEMIL);
    }

    #[test]
    fn test_rate_from_qty_vals() {
        let r =
            Rate::from_qty_vals(Amnt!(329.4) * FLOP, Amnt!(100) * MILLIEMIL);
        assert_eq!(r.term_amount(), Amnt!(329.4));
        assert_eq!(r.term_unit(), FLOP);
        assert_eq!(r.per_unit_multiple(), Amnt!(100));
        assert_eq!(r.per_unit(), MILLIEMIL);
    }

    #[test]
    fn test_rate_dimless() {
        let r = Rate::from_qty_vals(Amnt!(29.2), Amnt!(100));
        assert_eq!(r.term_amount(), Amnt!(29.2));
        assert_eq!(r.term_unit(), ONE);
        assert_eq!(r.per_unit_multiple(), Amnt!(100));
        assert_eq!(r.per_unit(), ONE);
    }

    #[test]
    fn test_rate_reciprocal() {
        let r =
            Rate::from_qty_vals(Amnt!(329.4) * FLOP, Amnt!(100) * MILLIEMIL);
        let r = r.reciprocal();
        assert_eq!(r.term_amount(), Amnt!(100));
        assert_eq!(r.term_unit(), MILLIEMIL);
        assert_eq!(r.per_unit_multiple(), Amnt!(329.4));
        assert_eq!(r.per_unit(), FLOP);
    }

    #[test]
    fn test_rate_mul_qty() {
        let r =
            Rate::from_qty_vals(Amnt!(329.4) * FLOP, Amnt!(100) * MILLIEMIL);
        let b = Amnt!(7.5) * EMIL;
        let f = r * b;
        assert_eq!(f.unit, r.term_unit());
        assert_almost_eq!(f.amount, Amnt!(24705));
        assert_eq!(f, b * r);
    }

    #[test]
    fn test_qty_div_rate() {
        let f = Amnt!(7.5) * KILOFLOP;
        let r = Rate::from_qty_vals(Amnt!(25) * FLOP, Amnt!(100) * MILLIEMIL);
        let b = f / r;
        assert_eq!(b.unit, r.per_unit());
        assert_almost_eq!(b.amount, Amnt!(30000));
    }

    #[test]
    fn test_rate_to_string() {
        let r =
            Rate::from_qty_vals(Amnt!(329.4) * FLOP, Amnt!(100) * MILLIEMIL);
        assert_eq!(r.to_string(), "329.4 f / 100 me");
        let r = Rate::from_qty_vals(Amnt!(329.4) * FLOP, Amnt!(1) * MILLIEMIL);
        assert_eq!(r.to_string(), "329.4 f / me");
        let r = Rate::from_qty_vals(Amnt!(4) * FLOP, Amnt!(1));
        assert_eq!(r.to_string(), "4 f / 1");
        let r = Rate::from_qty_vals(Amnt!(7.4) * FLOP, Amnt!(100));
        assert_eq!(r.to_string(), "7.4 f / 100");
    }
}
