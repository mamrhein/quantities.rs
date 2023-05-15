// ---------------------------------------------------------------------------
// Copyright:   (c) 2023 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use quantities::prelude::*;
    use serde_json;

    /// Foo, a completely useless quantity
    #[quantity]
    #[ref_unit(A, "aaa", MEGA)]
    #[unit(B, "b", 0.4)]
    #[unit(C, "c", CENTI, 0.01)]
    struct Foo {}

    #[test]
    fn test_unit() {
        let unit = FooUnit::B;
        let s = serde_json::to_value(unit).unwrap();
        assert_eq!(unit, serde_json::from_value::<FooUnit>(s).unwrap());
    }

    #[test]
    fn test_quantity() {
        let amnt = Amnt!(17.4);
        let unit = FooUnit::B;
        let qty = Foo::new(amnt, unit);
        let s = serde_json::to_value(qty).unwrap();
        assert_eq!(qty, serde_json::from_value::<Foo>(s).unwrap());
    }
}
