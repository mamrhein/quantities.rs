// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod quantity_with_ref_unit_tests {
    use quantities::assert_almost_eq;
    use quantities::prelude::*;

    /// Foo, a completely useless quantity
    #[quantity]
    #[ref_unit(A, "a", MEGA)]
    #[unit(B, "b", 0.4)]
    #[unit(C, "c", CENTI, 0.01)]
    struct Foo {}

    #[test]
    fn test_unit() {
        let a = A;
        let b = B;
        assert_eq!(a.name(), "A");
        assert_eq!(a.symbol(), "a");
        assert_eq!(a.si_prefix(), Some(SIPrefix::MEGA));
        assert_eq!(a.scale().unwrap(), Amnt!(1.0));
        assert_eq!(b.name(), "B");
        assert_eq!(b.symbol(), "b");
        assert_eq!(b.si_prefix(), None);
        assert_eq!(b.scale().unwrap(), Amnt!(0.4));
        assert_eq!(b.ratio(&a).unwrap(), Amnt!(0.4));
        assert_eq!(a.ratio(&b).unwrap(), Amnt!(2.5));
    }

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_unit_from_symbol() {
        assert_eq!(FooUnit::from_symbol("a"), Some(A));
        assert_eq!(FooUnit::from_symbol("b"), Some(B));
        assert_eq!(FooUnit::from_symbol("c"), Some(C));
        assert_eq!(FooUnit::from_symbol("x"), None);
    }

    #[test]
    fn test_unit_from_scale() {
        assert_eq!(FooUnit::from_scale(Amnt!(1)), Some(A));
        assert_eq!(FooUnit::from_scale(Amnt!(0.4)), Some(B));
        assert_eq!(FooUnit::from_scale(Amnt!(0.01)), Some(C));
        assert_eq!(FooUnit::from_scale(Amnt!(10)), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_fit() {
        let amnt = Amnt!(0.007);
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.amount, amnt / Amnt!(0.01));
        assert_eq!(foo.unit, C);
        let amnt = Amnt!(0.07);
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.amount, amnt / Amnt!(0.01));
        assert_eq!(foo.unit, C);
        let amnt = Amnt!(0.7);
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.amount, amnt / Amnt!(0.01));
        assert_eq!(foo.unit, C);
        let amnt = Amnt!(7.);
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.amount, amnt);
        assert_eq!(foo.unit, A);
    }

    #[test]
    fn test_qty_unit_from_symbol() {
        assert_eq!(Foo::unit_from_symbol("a"), Some(A));
        assert_eq!(Foo::unit_from_symbol("b"), Some(B));
        assert_eq!(Foo::unit_from_symbol("c"), Some(C));
        assert_eq!(Foo::unit_from_symbol("x"), None);
    }

    #[test]
    fn test_qty_unit_from_scale() {
        assert_eq!(Foo::unit_from_scale(Amnt!(1)), Some(A));
        assert_eq!(Foo::unit_from_scale(Amnt!(0.4)), Some(B));
        assert_eq!(Foo::unit_from_scale(Amnt!(0.01)), Some(C));
        assert_eq!(Foo::unit_from_scale(Amnt!(10)), None);
    }

    #[test]
    fn test_qty() {
        let amnt = Amnt!(17.4);
        let unit = FooUnit::B;
        let qty = Foo::new(amnt, unit);
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
        let qty = amnt * unit;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
    }

    #[test]
    fn test_qty_to_string() {
        let qty = Foo::new(Amnt!(184.09), FooUnit::A);
        assert_eq!(qty.to_string(), "184.09 a");
    }

    #[test]
    fn test_convert() {
        let qty = Foo::new(Amnt!(17.4), FooUnit::B);
        let equiv = qty.convert(FooUnit::A).unwrap();
        assert_almost_eq!(equiv.amount(), Amnt!(6.96));
        assert_eq!(equiv.unit(), FooUnit::A);
        let qty = equiv.convert(FooUnit::B).unwrap();
        assert_almost_eq!(qty.amount(), Amnt!(17.4));
        assert_eq!(qty.unit(), FooUnit::B);
    }

    #[test]
    fn test_cmp_same_unit() {
        let qty1 = Amnt!(17.4) * FooUnit::A;
        let qty2 = Amnt!(0.37) * FooUnit::A;
        let qty3 = qty1;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[test]
    fn test_cmp_diff_unit() {
        let qty1 = Amnt!(17.4) * FooUnit::A;
        let qty2 = Amnt!(0.37) * FooUnit::B;
        let qty3 = qty1.convert(FooUnit::C).unwrap();
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = Amnt!(0.1);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(-0.2);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_almost_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    fn test_add_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 + qty2;
        assert_almost_eq!(res.amount(), amnt1 + amnt2 * Amnt!(0.4));
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.amount(), amnt1 * Amnt!(2.5) + amnt2);
        assert_eq!(res.unit(), unit2);
    }

    #[test]
    fn test_sub_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 - qty2;
        assert_almost_eq!(res.amount(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.amount(), amnt2 - amnt1);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    fn test_sub_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.3);
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 - qty2;
        assert_almost_eq!(res.amount(), amnt1 - amnt2 * Amnt!(0.4));
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.amount(), amnt2 - amnt1 * Amnt!(2.5));
        assert_eq!(res.unit(), unit2);
    }

    #[test]
    fn test_div_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.3);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_almost_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), ONE);
    }

    #[test]
    fn test_div_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.3);
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 / qty2;
        assert_almost_eq!(res.amount(), amnt1 / (amnt2 * Amnt!(0.4)));
        assert_eq!(res.unit(), ONE);
    }

    #[test]
    fn test_mul_amnt() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let res = qty1 * amnt2;
        assert_almost_eq!(res.amount(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
        let res = amnt2 * qty1;
        assert_almost_eq!(res.amount(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
    }

    #[test]
    fn test_div_amnt() {
        let amnt1 = Amnt!(15.54);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(3.7);
        let qty1 = amnt1 * unit1;
        let res = qty1 / amnt2;
        assert_almost_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), qty1.unit());
    }
}

#[cfg(test)]
mod quantity_without_ref_unit_tests {
    use quantities::assert_almost_eq;
    use quantities::prelude::*;

    /// Foo, a completely useless quantity
    #[quantity]
    #[unit(A, "a")]
    #[unit(B, "b")]
    #[unit(C, "c")]
    struct Foo {}

    #[test]
    fn test_unit() {
        let a = A;
        let b = B;
        assert_eq!(b.name(), "B");
        assert_eq!(b.symbol(), "b");
        assert!(b.si_prefix().is_none());
        assert!(b.scale().is_none());
        assert!(b.ratio(&a).is_none());
        assert!(a.ratio(&b).is_none());
    }

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_convert() {
        let qty = Foo::new(Amnt!(17.4), B);
        assert!(qty.convert(A).is_none());
        let qty = Foo::new(Amnt!(6.25), A);
        assert!(qty.convert(B).is_none());
    }

    #[test]
    fn test_cmp_same_unit() {
        let qty1 = Amnt!(17.4) * FooUnit::A;
        let qty2 = Amnt!(0.37) * FooUnit::A;
        let qty3 = qty1;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[test]
    fn test_cmp_diff_unit() {
        let qty1 = Amnt!(17.4) * FooUnit::A;
        let qty2 = Amnt!(0.37) * FooUnit::B;
        let qty3 = Amnt!(17.4000) * FooUnit::A;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(!(qty2 < qty1));
        assert!(!(qty1 > qty2));
        assert!(!(qty2 <= qty3));
        assert!(!(qty3 >= qty2));
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = Amnt!(0.1);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.2);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_almost_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    #[should_panic]
    fn test_add_diff_unit() {
        let qty1 = Amnt!(17.4) * A;
        let qty2 = Amnt!(0.37) * B;
        let _res = qty1 + qty2;
    }

    #[test]
    fn test_sub_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 - qty2;
        assert_almost_eq!(res.amount(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.amount(), amnt2 - amnt1);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    #[should_panic]
    fn test_sub_diff_unit() {
        let qty1 = Amnt!(17.4) * A;
        let qty2 = Amnt!(0.37) * B;
        let _res = qty1 - qty2;
    }

    #[test]
    fn test_div_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.3);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_almost_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), ONE);
    }

    #[test]
    #[should_panic]
    fn test_div_diff_unit() {
        let qty1 = Amnt!(17.4) * A;
        let qty2 = Amnt!(0.3) * B;
        let _res = qty1 / qty2;
    }
}

#[cfg(test)]
mod quantity_single_unit_tests {
    use quantities::prelude::*;

    #[quantity]
    #[unit(Pop, "p")]
    struct Foo {}

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&POP));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&POP));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_single_unit_qty() {
        let amnt = Amnt!(17.4);
        let qty = Foo::new(amnt, POP);
        assert_eq!(qty.amount(), amnt);
        let qty = amnt * POP;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), POP);
        let qty = POP * amnt;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), POP);
    }
}

#[cfg(test)]
mod derived_quantity_tests {
    use quantities::assert_almost_eq;
    use quantities::prelude::*;

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

    #[quantity(Foo * Bar)]
    #[ref_unit(Bazoo, "b", "1·f·e")]
    #[unit(Millibazoo, "mb", 0.001, "0.001·b")]
    #[unit(Microbazoo, "µb", 0.000001, "0.000001·b")]
    #[unit(Kilobazoo, "kb", 1000., "1000·b")]
    struct Baz {}

    #[quantity(Foo / Bar)]
    #[ref_unit(Qoox, "Q", "1·f/e")]
    #[unit(Five_Flops_per_Emil, "ff/e", 5., "5·f/e")]
    #[unit(Milliqoox, "mQ", 0.001, "0.001·Q")]
    #[unit(Microqoox, "µQ", 0.000001, "0.000001·Q")]
    #[unit(Kiloqoox, "kQ", 1000., "1000·Q")]
    struct Qoo {}

    #[test]
    fn test_qty() {
        let amnt = Amnt!(17.4);
        let unit = BazUnit::Microbazoo;
        let qty = Baz::new(amnt, unit);
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
        let qty = amnt * unit;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
    }

    fn check_qty_mul_qty(x: Foo, y: Bar, r: Baz) {
        let z = x * y;
        assert_almost_eq!(z.amount(), r.amount());
        assert_eq!(z.unit(), r.unit());
        let z = y * x;
        assert_almost_eq!(z.amount(), r.amount());
        assert_eq!(z.unit(), r.unit());
        // reverse divs
        let z = (r / x).convert(y.unit()).unwrap();
        assert_almost_eq!(z.amount(), y.amount());
        assert_eq!(z.unit(), y.unit());
        let z = (r / y).convert(x.unit()).unwrap();
        assert_almost_eq!(z.amount(), x.amount());
        assert_eq!(z.unit(), x.unit());
    }

    #[test]
    fn test_qty_mul_qty() {
        check_qty_mul_qty(
            Amnt!(17.4) * FLOP,
            Amnt!(3.) * EMIL,
            Amnt!(17.4) * Amnt!(3.) * BAZOO,
        );
        check_qty_mul_qty(
            Amnt!(14.52) * KILOFLOP,
            Amnt!(0.47) * MICROEMIL,
            Amnt!(14.52) * Amnt!(0.47) * MILLIBAZOO,
        );
        check_qty_mul_qty(
            Amnt!(14.52) * CENTIFLOP,
            Amnt!(0.47) * MICROEMIL,
            Amnt!(14.52) * Amnt!(0.47) * Amnt!(0.01) * MICROBAZOO,
        );
    }

    fn check_qty_div_qty(x: Foo, y: Bar, r: Qoo) {
        let z = x / y;
        assert_almost_eq!(z.amount(), r.amount());
        assert_eq!(z.unit(), r.unit());
        // reverse mul
        let z = (r * y).convert(x.unit()).unwrap();
        assert_almost_eq!(z.amount(), x.amount());
        assert_eq!(z.unit(), x.unit());
    }

    #[test]
    fn test_qty_div_qty() {
        check_qty_div_qty(
            Amnt!(17.4) * FLOP,
            Amnt!(3.) * EMIL,
            Amnt!(17.4) / Amnt!(3.) * QOOX,
        );
        check_qty_div_qty(
            Amnt!(14.52) * KILOFLOP,
            Amnt!(3.3) * MILLIEMIL,
            (Amnt!(14.52) / Amnt!(3.3)) * Amnt!(1000.) * KILOQOOX,
        );
        check_qty_div_qty(
            Amnt!(14.52) * CENTIFLOP,
            Amnt!(3.3) * KILOEMIL,
            Amnt!(14.52) / Amnt!(3.3) * Amnt!(10.) * MICROQOOX,
        );
    }
}
