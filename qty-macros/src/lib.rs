// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#![doc = include_str ! ("../README.md")]

mod quantity_attr_helper;

use ::convert_case::{Case, Casing};
use ::proc_macro::TokenStream;
use ::proc_macro2::{Span, TokenStream as TokenStream2};
use ::proc_macro_error::proc_macro_error;
use ::quote::quote;
use ::syn::{parse_macro_input, Ident, ItemEnum};

use crate::quantity_attr_helper::{analyze, codegen, parse_args, parse_item};

/// Derives a constant for each variant of a fieldless enum.
///
/// # Panics
///
/// The macro panics in the following cases:
///
/// * The attributed item is not an enum.
/// * The enum is not a fieldless enum (aka C-like enum).
///
/// # Example
///
/// ```rust
/// # use qty_macros::VariantsAsConstants;
/// # #[allow(non_camel_case_types)]
/// #[derive(VariantsAsConstants)]
/// enum TestEnum {
///     MultiCamelCase,
///     snake_case,
///     simple,
///     ALL_UPPER,
/// }
/// ```
///
/// This results in the following additional code:
///
/// ```rust
/// # #[allow(non_camel_case_types)]
/// # enum TestEnum {
/// #     MultiCamelCase,
/// #     snake_case,
/// #     simple,
/// #     ALL_UPPER,
/// # }
/// pub const MULTI_CAMEL_CASE: TestEnum = TestEnum::MultiCamelCase;
/// pub const SNAKE_CASE: TestEnum = TestEnum::snake_case;
/// pub const SIMPLE: TestEnum = TestEnum::simple;
/// pub const ALL_UPPER: TestEnum = TestEnum::ALL_UPPER;
/// ```
#[proc_macro_derive(VariantsAsConstants)]
pub fn derive_variants_as_constants(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let mut output = TokenStream2::new();
    let enum_ident = enum_def.ident;
    // check and gather const declarations for variants
    for variant in enum_def.variants {
        if !variant.fields.is_empty() {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_ident = variant.ident;
        let const_ident = Ident::new(
            variant_ident.to_string().to_case(Case::UpperSnake).as_str(),
            Span::call_site(),
        );
        output = quote!(
            #output
            pub const #const_ident: #enum_ident = #enum_ident::#variant_ident;
        );
    }
    output.into()
}

/// Derives a function `iter` that returns an iterator over the variants of a
/// fieldless enum.
///
/// # Panics
///
/// The macro panics in the following cases:
///
/// * The attributed item is not an enum.
/// * The enum is not a fieldless enum (aka C-like enum).
///
/// # Example
///
/// ```rust
/// # use qty_macros::VariantsAsConstants;
/// #[derive(VariantsAsConstants)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
/// ```
///
/// This results in the following additional code:
///
/// ```rust
/// # enum Color {
/// #     Red,
/// #     Green,
/// #     Blue,
/// # }
/// impl Color {
///     const VARIANTS: [Self; 3usize] =
///         [Self::Red, Self::Green, Self::Blue];
///     #[doc = "Returns an iterator over the variants of `Self`."]
///     #[inline(always)]
///     pub fn iter() -> core::slice::Iter<'static, Self> {
///         Self::VARIANTS.iter()
///     }
/// }
/// ```
#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let enum_ident = enum_def.ident;
    let mut output = TokenStream2::new();
    // check and gather variants
    for variant in &enum_def.variants {
        if !variant.fields.is_empty() {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_ident = &variant.ident;
        output = quote!(
            #output
            Self::#variant_ident,
        );
    }
    let n_variants = &enum_def.variants.len();
    // create impl for fn iter
    output = quote!(
        impl #enum_ident {
            const VARIANTS: [Self; #n_variants] = [#output];
            #[doc = "Returns an iterator over the variants of `Self`."]
            #[inline(always)]
            pub fn iter() -> core::slice::Iter<'static, Self> {
                Self::VARIANTS.iter()
            }
        }
    );
    output.into()
}

/// Generates an enum with the given units (incl. the refunit, if given) as
/// variants, an implemention of trait `Unit` for this enum and a type alias of
/// `Qty` with the enum as parameter and named after the given struct.
///
/// In addition, it creates a constant for each enum variant, thus providing a
/// constant for each unit.
///
/// This implies that the identifiers of all units over all defined
/// quantitities have to be unique!
///
/// The attribute `#[quantity]` can optionally be followed by an attribute
/// `#[ref_unit]` and must be followed by at least one attribute `#[unit]`.
///
/// To define a quantity with a reference unit, use one of the following forms
/// of the ref_unit attribute
///
/// `#[ref_unit(<ident>, "<symbol>", <si_prefix>, "<doc>")]`
/// `#[ref_unit(<ident>, "<symbol>", <si_prefix>)]`
/// `#[ref_unit(<ident>, "<symbol>", "<doc>")]`,
/// `#[ref_unit(<ident>, "<symbol>")]`,
///
/// followed by one ore more unit attributes in one of the following forms
///
/// `#[unit(<ident>, "<symbol>", <si_prefix>, <scale>, "<doc>")]`
/// `#[unit(<ident>, "<symbol>", <si_prefix>, <scale>)]`
/// `#[unit(<ident>, "<symbol>", <scale>, "<doc>")]`.
/// `#[unit(<ident>, "<symbol>", <scale>)]`.
///
/// To define a quantity without a reference unit, use one ore more unit
/// attributes in one of the following forms
///
/// `#[unit(<ident>, "<symbol>", "<doc>")]`
/// `#[unit(<ident>, "<symbol>")]`.
///
/// # Panics
///
/// The macro panics in the followong cases:
///
/// * Invalid arguments given to the attribute `#[quantity]`.
/// * The given item is not a struct.
/// * The given struct does have generic parameters and/or fields.
/// * More than one attribute `#[ref_unit]` is given.
/// * No attribute `#[unit]` is given.
/// * Wrong number or wrong type of arguments given to attribute `#[ref_unit]`.
/// * Wrong number of arguments given to an attribute `#[unit]`.
/// * No \<scale\> argument given to an attribute `#[unit]` when required.
///
/// # Example
///
/// ```compile_fail
/// use quantities::prelude::*; // This dependency can't be fulfilled here!
/// #[quantity]
/// #[ref_unit(Kilogram, "kg", KILO, "Reference unit of quantity `Mass`")]
/// #[unit(Milligram, "mg", MILLI, 0.000001, "0.001·g")]
/// #[unit(Gram, "g", NONE, 0.001, "0.001·kg")]
/// #[unit(Ounce, "oz", 0.028349523125, "0.0625·lb")]
/// #[unit(Pound, "lb", 0.45359237, "0.45359237·kg")]
/// #[unit(Tonne, "t", MEGA, 1000, "1000·kg")]
/// /// The quantity of matter in a physical body.
/// struct Mass {}
/// ```
///
/// This results in the following code:
///
/// ```compile_fail
/// #[doc = " The quantity of matter in a physical body."]
/// #[derive(Copy, Clone, Debug)]
/// pub struct Mass {
///     amount: AmountT,
///     unit: MassUnit,
/// }
/// impl Quantity for Mass {
///     type UnitType = MassUnit;
///     #[inline(always)]
///     fn new(amount: AmountT, unit: Self::UnitType) -> Self {
///         Self { amount, unit }
///     }
///     #[inline(always)]
///     fn amount(&self) -> AmountT {
///         self.amount
///     }
///     #[inline(always)]
///     fn unit(&self) -> Self::UnitType {
///         self.unit
///     }
/// }
/// #[doc = "Unit of quantity `Mass`."]
/// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// pub enum MassUnit {
///     #[doc = "0.001·g"]
///     Milligram,
///     #[doc = "0.001·kg"]
///     Gram,
///     #[doc = "0.0625·lb"]
///     Ounce,
///     #[doc = "0.45359237·kg"]
///     Pound,
///     #[doc = "Reference unit of quantity `Mass`"]
///     Kilogram,
///     #[doc = "1000·kg"]
///     Tonne,
/// }
/// impl MassUnit {
///     const VARIANTS: [MassUnit; 6usize] = [
///         MassUnit::Milligram,
///         MassUnit::Gram,
///         MassUnit::Ounce,
///         MassUnit::Pound,
///         MassUnit::Kilogram,
///         MassUnit::Tonne,
///     ];
/// }
/// impl Unit for MassUnit {
///     type QuantityType = Mass;
///     fn iter<'a>() -> core::slice::Iter<'a, Self> {
///         Self::VARIANTS.iter()
///     }
///     fn name(&self) -> &'static str {
///         match self {
///             MassUnit::Milligram => "Milligram",
///             MassUnit::Gram => "Gram",
///             MassUnit::Ounce => "Ounce",
///             MassUnit::Pound => "Pound",
///             MassUnit::Kilogram => "Kilogram",
///             MassUnit::Tonne => "Tonne",
///         }
///     }
///     fn symbol(&self) -> &'static str {
///         match self {
///             MassUnit::Milligram => "mg",
///             MassUnit::Gram => "g",
///             MassUnit::Ounce => "oz",
///             MassUnit::Pound => "lb",
///             MassUnit::Kilogram => "kg",
///             MassUnit::Tonne => "t",
///         }
///     }
///     fn si_prefix(&self) -> Option<SIPrefix> {
///         match self {
///             MassUnit::Milligram => Some(SIPrefix::MILLI),
///             MassUnit::Gram => Some(SIPrefix::NONE),
///             MassUnit::Kilogram => Some(SIPrefix::KILO),
///             MassUnit::Tonne => Some(SIPrefix::MEGA),
///             _ => None,
///         }
///     }
/// }
/// impl LinearScaledUnit for MassUnit {
///     const REF_UNIT: Self = MassUnit::Kilogram;
///     fn scale(&self) -> AmountT {
///         match self {
///             MassUnit::Milligram => 0.000001 as f64,
///             MassUnit::Gram => 0.001 as f64,
///             MassUnit::Ounce => 0.028349523125 as f64,
///             MassUnit::Pound => 0.45359237 as f64,
///             MassUnit::Kilogram => 1.0 as f64,
///             MassUnit::Tonne => 1000 as f64,
///         }
///     }
/// }
/// impl HasRefUnit for Mass {
///     const REF_UNIT: MassUnit = MassUnit::Kilogram;
/// }
/// impl Eq for Mass {}
/// impl PartialEq<Self> for Mass {
///     #[inline(always)]
///     fn eq(&self, other: &Self) -> bool {
///         <Self as HasRefUnit>::eq(self, other)
///     }
/// }
/// impl PartialOrd for Mass {
///     #[inline(always)]
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///         <Self as HasRefUnit>::partial_cmp(self, other)
///     }
/// }
/// impl Add<Self> for Mass {
///     type Output = Self;
///     #[inline(always)]
///     fn add(self, rhs: Self) -> Self::Output {
///         <Self as HasRefUnit>::add(self, rhs)
///     }
/// }
/// impl Sub<Self> for Mass {
///     type Output = Self;
///     #[inline(always)]
///     fn sub(self, rhs: Self) -> Self::Output {
///         <Self as HasRefUnit>::sub(self, rhs)
///     }
/// }
/// impl Div<Self> for Mass {
///     type Output = AmountT;
///     #[inline(always)]
///     fn div(self, rhs: Self) -> Self::Output {
///         <Self as HasRefUnit>::div(self, rhs)
///     }
/// }
/// #[doc = "0.001·g"]
/// pub const MILLIGRAM: MassUnit = MassUnit::Milligram;
/// #[doc = "0.001·kg"]
/// pub const GRAM: MassUnit = MassUnit::Gram;
/// #[doc = "0.0625·lb"]
/// pub const OUNCE: MassUnit = MassUnit::Ounce;
/// #[doc = "0.45359237·kg"]
/// pub const POUND: MassUnit = MassUnit::Pound;
/// #[doc = "Reference unit of quantity `Mass`"]
/// pub const KILOGRAM: MassUnit = MassUnit::Kilogram;
/// #[doc = "1000·kg"]
/// pub const TONNE: MassUnit = MassUnit::Tonne;
/// impl Mul<MassUnit> for AmountT {
///     type Output = Mass;
///     #[inline(always)]
///     fn mul(self, rhs: MassUnit) -> Self::Output {
///         Mass::new(self, rhs)
///     }
/// }
/// impl Mul<AmountT> for MassUnit {
///     type Output = Mass;
///     #[inline(always)]
///     fn mul(self, rhs: AmountT) -> Self::Output {
///         Mass::new(rhs, self)
///     }
/// }
/// impl fmt::Display for Mass {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         <Self as Quantity>::fmt(self, f)
///     }
/// }
/// impl Mul<Mass> for AmountT {
///     type Output = Mass;
///     #[inline(always)]
///     fn mul(self, rhs: Mass) -> Self::Output {
///         Self::Output::new(self * rhs.amount(), rhs.unit())
///     }
/// }
/// impl Mul<AmountT> for Mass {
///     type Output = Self;
///     #[inline(always)]
///     fn mul(self, rhs: AmountT) -> Self::Output {
///         Self::Output::new(self.amount() * rhs, self.unit())
///     }
/// }
/// impl Div<AmountT> for Mass {
///     type Output = Self;
///     #[inline(always)]
///     fn div(self, rhs: AmountT) -> Self::Output {
///         Self::Output::new(self.amount() / rhs, self.unit())
///     }
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn quantity(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_ast = parse_item(item.into());
    let mut qty_def = analyze(&mut item_ast);
    qty_def.derived_as = parse_args(args.into());
    let code = codegen(&qty_def, &item_ast.attrs);
    code.into()
}
