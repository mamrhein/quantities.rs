// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

mod quantity_attr_helper;

use crate::quantity_attr_helper::{analyze, codegen, parse};
use ::convert_case::{Case, Casing};
use ::proc_macro::TokenStream;
use ::proc_macro2::{Span, TokenStream as TokenStream2};
use ::proc_macro_error::proc_macro_error;
use ::quote::quote;
use ::syn::{parse_macro_input, Ident, ItemEnum};

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
            #enum_ident::#variant_ident,
        );
    }
    let n_variants = &enum_def.variants.len();
    // create impl for fn iter
    output = quote!(
        impl #enum_ident {
            const VARIANTS: [#enum_ident; #n_variants] = [#output];
            #[doc = "Returns an iterator over the variants of `Self`."]
            #[inline(always)]
            pub fn iter() -> core::slice::Iter<'static, Self> {
                Self::VARIANTS.iter()
            }
        }
    );
    output.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn quantity(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse(args.into(), item.into());
    let qty_def = analyze(&mut ast);
    let code = codegen(&qty_def, &ast.attrs);
    code.into()
}
