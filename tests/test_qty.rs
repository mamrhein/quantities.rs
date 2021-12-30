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
    use core::ops::Mul;
    use quantities::{
        quantity, Amnt, AmountT, Qty, Quantity, SIPrefix, Unit, NON_UNIT,
    };

    /// Foo, a completely useless quantity
    #[quantity]
    #[ref_unit(A, "a")]
    #[unit(B, "b", 0.4)]
    struct Foo {}

    #[test]
    fn test_unit() {
        let a = A;
        let b = B;
        assert_eq!(b.name(), "B");
        assert_eq!(b.symbol(), "b");
        assert_eq!(b.si_prefix(), None);
        assert_eq!(b.scale().unwrap(), Amnt!(0.4));
        assert_eq!(b.ratio(&a).unwrap(), Amnt!(0.4));
        assert_eq!(a.ratio(&b).unwrap(), Amnt!(2.5));
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

    #[cfg(feature = "std")]
    #[test]
    fn test_qty_to_string() {
        let qty = Foo::new(Amnt!(184.09), FooUnit::A);
        assert_eq!(qty.to_string(), "184.09 a");
    }

    #[test]
    fn test_convert() {
        let qty = Foo::new(Amnt!(17.4), FooUnit::B);
        let equiv = qty.convert(FooUnit::A).unwrap();
        assert_eq!(equiv.amount(), Amnt!(6.96));
        assert_eq!(equiv.unit(), FooUnit::A);
        let qty = equiv.convert(FooUnit::B).unwrap();
        assert_eq!(qty.amount(), Amnt!(17.4));
        assert_eq!(qty.unit(), FooUnit::B);
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_eq!(res.amount(), amnt1 + amnt2);
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
        assert_eq!(res.amount(), amnt1 + amnt2 * Amnt!(0.4));
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_eq!(res.amount(), amnt1 * Amnt!(2.5) + amnt2);
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
        assert_eq!(res.amount(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_eq!(res.amount(), amnt2 - amnt1);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    fn test_sub_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 - qty2;
        assert_eq!(res.amount(), amnt1 - amnt2 * Amnt!(0.4));
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_eq!(res.amount(), amnt2 - amnt1 * Amnt!(2.5));
        assert_eq!(res.unit(), unit2);
    }

    #[test]
    fn test_div_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), NON_UNIT);
    }

    #[test]
    fn test_div_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 / qty2;
        assert_eq!(res.amount(), amnt1 / (amnt2 * Amnt!(0.4)));
        assert_eq!(res.unit(), NON_UNIT);
    }

    #[test]
    fn test_mul_amnt() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let res = qty1 * amnt2;
        assert_eq!(res.amount(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
        let res = amnt2 * qty1;
        assert_eq!(res.amount(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
    }
}

#[cfg(test)]
mod quantity_without_ref_unit_tests {
    use core::ops::Mul;
    use quantities::{
        quantity, Amnt, AmountT, Qty, Quantity, SIPrefix, Unit, NON_UNIT,
    };

    /// Foo, a completely useless quantity
    #[quantity]
    #[unit(A, "a")]
    #[unit(B, "b")]
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
    fn test_convert() {
        let qty = Foo::new(Amnt!(17.4), B);
        assert!(qty.convert(A).is_none());
        let qty = Foo::new(Amnt!(6.25), A);
        assert!(qty.convert(B).is_none());
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = FooUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_eq!(res.amount(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_eq!(res.amount(), amnt1 + amnt2);
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
        assert_eq!(res.amount(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_eq!(res.amount(), amnt2 - amnt1);
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
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), NON_UNIT);
    }

    #[test]
    #[should_panic]
    fn test_div_diff_unit() {
        let qty1 = Amnt!(17.4) * A;
        let qty2 = Amnt!(0.37) * B;
        let _res = qty1 / qty2;
    }
}

#[cfg(test)]
mod quantity_single_unit_tests {
    use core::ops::Mul;
    use quantities::{quantity, Amnt, AmountT, Qty, Quantity, SIPrefix, Unit};

    #[quantity]
    #[unit(Pop, "p")]
    struct Foo {}

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
