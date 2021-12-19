// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod tests {
    use core::ops::Mul;
    use quantities::{
        define_qty, impl_mul_amnt_unit, opt, Amnt, AmountT, Qty, Quantity,
        SIPrefix, Unit, NONUNIT,
    };

    define_qty!(
        TestQty,
        TestUnit,
        A,
        (A, "A", "a", None, 1.0),
        (B, "B", "b", None, 0.4)
    );

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
        let unit = TestUnit::B;
        let qty = TestQty::new(amnt, unit);
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
        let qty = amnt * unit;
        assert_eq!(qty.amount(), amnt);
        assert_eq!(qty.unit(), unit);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_qty_to_string() {
        let qty = TestQty::new(Amnt!(184.09), TestUnit::A);
        assert_eq!(qty.to_string(), "184.09 a");
    }

    #[test]
    fn test_convert() {
        let qty = TestQty::new(Amnt!(17.4), TestUnit::B);
        let equiv = qty.convert(TestUnit::A).unwrap();
        assert_eq!(equiv.amount(), Amnt!(6.96));
        assert_eq!(equiv.unit(), TestUnit::A);
        let qty = equiv.convert(TestUnit::B).unwrap();
        assert_eq!(qty.amount(), Amnt!(17.4));
        assert_eq!(qty.unit(), TestUnit::B);
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = TestUnit::A;
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
        let unit1 = TestUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = TestUnit::B;
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
        let unit1 = TestUnit::A;
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
        let unit1 = TestUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = TestUnit::B;
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
        let unit1 = TestUnit::A;
        let amnt2 = Amnt!(0.37);
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_eq!(res.amount(), amnt1 / amnt2);
        assert_eq!(res.unit(), NONUNIT);
    }

    #[test]
    fn test_div_diff_unit() {
        let amnt1 = Amnt!(17.4);
        let unit1 = TestUnit::A;
        let amnt2 = Amnt!(0.37);
        let unit2 = TestUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 / qty2;
        assert_eq!(res.amount(), amnt1 / (amnt2 * Amnt!(0.4)));
        assert_eq!(res.unit(), NONUNIT);
    }

    #[test]
    fn test_mul_amnt() {
        let amnt1 = Amnt!(17.4);
        let unit1 = TestUnit::A;
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
