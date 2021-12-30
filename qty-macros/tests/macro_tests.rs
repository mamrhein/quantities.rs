// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod derive_macro_tests {
    use qty_macros::{EnumIter, VariantsAsConstants};

    #[test]
    #[allow(non_camel_case_types)]
    fn test_derive_constants() {
        #[derive(Debug, PartialEq, VariantsAsConstants)]
        pub enum TestEnum {
            MultiCamelCase,
            snake_case,
            simple,
            ALL_UPPER,
        }
        assert_eq!(MULTI_CAMEL_CASE, TestEnum::MultiCamelCase);
        assert_eq!(SNAKE_CASE, TestEnum::snake_case);
        assert_eq!(SIMPLE, TestEnum::simple);
        assert_eq!(ALL_UPPER, TestEnum::ALL_UPPER);
    }

    #[test]
    fn test_derive_enum_iter() {
        #[derive(Debug, PartialEq, EnumIter)]
        pub enum Color {
            Red,
            Green,
            Blue,
        }
        let mut it = Color::iter();
        assert_eq!(it.next(), Some(&Color::Red));
        assert_eq!(it.next(), Some(&Color::Green));
        assert_eq!(it.next(), Some(&Color::Blue));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
