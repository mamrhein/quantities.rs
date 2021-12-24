// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use ::convert_case::{Case, Casing};
use ::proc_macro::TokenStream;
use ::proc_macro2::{Span, TokenStream as TokenStream2};
use ::quote::quote;
use ::syn::{parse_macro_input, Ident, ItemEnum};

#[proc_macro_derive(VariantsAsConstants)]
pub fn derive_variants_as_constants(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let mut output = TokenStream2::new();
    let enum_id = enum_def.ident;
    // check and gather const declarations for variants
    for variant in enum_def.variants {
        if variant.fields.len() != 0 {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_id = variant.ident;
        let const_id = Ident::new(
            variant_id.to_string().to_case(Case::UpperSnake).as_str(),
            Span::call_site(),
        );
        output = quote!(
            #output
            pub const #const_id: #enum_id = #enum_id::#variant_id;
        );
    }
    output.into()
}

#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let enum_id = enum_def.ident;
    let mut output = TokenStream2::new();
    // check and gather variants
    for variant in &enum_def.variants {
        if variant.fields.len() != 0 {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_id = &variant.ident;
        output = quote!(
            #output
            #enum_id::#variant_id,
        );
    }
    let n_variants = &enum_def.variants.len();
    // create impl for fn iter
    output = quote!(
        impl #enum_id {
            const VARIANTS: [#enum_id; #n_variants] = [#output];
            #[doc = "Returns an iterator over the variants of `Self`."]
            #[inline(always)]
            pub fn iter() -> core::slice::Iter<'static, Self> {
                Self::VARIANTS.iter()
            }
        }
    );
    output.into()
}
