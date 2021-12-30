// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn;

pub(crate) struct UnitDef {
    unit_ident: syn::Ident,
    name: syn::LitStr,
    symbol: syn::LitStr,
    si_prefix: Option<syn::Ident>,
    scale: Option<syn::LitFloat>,
}

pub(crate) struct QtyDef {
    qty_ident: syn::Ident,
    ref_unit_ident: Option<syn::Ident>,
    units: Vec<UnitDef>,
}

impl QtyDef {
    fn new(qty_id: syn::Ident) -> Self {
        Self {
            qty_ident: qty_id,
            ref_unit_ident: None,
            units: vec![],
        }
    }
}

pub(crate) type Ast = syn::ItemStruct;

pub(crate) fn parse(args: TokenStream, item: TokenStream) -> Ast {
    const ERROR: &str = "Attribute `quantity` takes no arguments.";
    const HELP: &str = "Use `#[quantity]`.";

    if !args.is_empty() {
        if let Ok(expr) = syn::parse2::<syn::Expr>(args.into()) {
            abort!(expr, ERROR; help = HELP)
        } else {
            abort_call_site!(ERROR; help = HELP)
        }
    }
    match syn::parse2::<Ast>(item.clone()) {
        Ok(item) => item,
        Err(error) => abort!(item, error; help = HELP),
    }
}

fn check_struct(ast: &Ast) {
    const GENERICS_ERROR: &str =
        "Given struct must not have generic parameters.";
    const FIELDS_ERROR: &str = "Given struct must not have fields.";
    let help = format!("Use `struct {};`", ast.ident);

    if ast.generics.params.len() != 0 {
        abort!(ast.generics, GENERICS_ERROR; help = help.as_str());
    }
    if ast.fields.len() != 0 {
        abort!(ast.fields, FIELDS_ERROR; help = help.as_str());
    }
}

#[inline]
fn is_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path
        .is_ident(&syn::Ident::new("unit", Span::call_site()))
}

#[inline]
fn is_ref_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path
        .is_ident(&syn::Ident::new("ref_unit", Span::call_site()))
}

const ARGS_LIST_ERROR: &str =
    "A comma-separated list of 2 to 4 arguments expected.";

const UNIT_ATTR_HELP: &str =
    "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\")]`.";

fn get_unit_attrs(
    attrs: &Vec<syn::Attribute>,
) -> (Vec<syn::Attribute>, Option<syn::Attribute>) {
    const ERROR: &str =
        "At least one unit description must be given via attribute `unit`.";

    let mut unit_attrs: Vec<syn::Attribute> = vec![];
    let mut opt_ref_unit_attr: Option<syn::Attribute> = None;
    for attr in attrs {
        if is_unit_attr(attr) {
            unit_attrs.push(attr.clone());
        } else if is_ref_unit_attr(attr) {
            opt_ref_unit_attr = Some(attr.clone());
        }
    }
    if unit_attrs.len() == 0 {
        abort_call_site!(ERROR; help = UNIT_ATTR_HELP);
    }
    (unit_attrs, opt_ref_unit_attr)
}

impl syn::parse::Parse for UnitDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let unit_ident: syn::Ident = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let symbol: syn::LitStr = input.parse()?;
        let opt_comma: Option<syn::Token![,]> = input.parse()?;
        if opt_comma.is_none() && !input.is_empty() {
            return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
        }
        let mut si_prefix: Option<syn::Ident> = None;
        if input.peek(syn::Ident) {
            si_prefix = Some(input.parse::<syn::Ident>()?);
            let opt_comma: Option<syn::Token![,]> = input.parse()?;
            if opt_comma.is_none() && !input.is_empty() {
                return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
            }
        };
        let mut scale: Option<syn::LitFloat> = None;
        if input.peek(syn::LitFloat) {
            scale = Some(input.parse::<syn::LitFloat>()?);
        };
        // Allow trailing comma:
        let _: Option<syn::Token![,]> = input.parse()?;
        // Check if input is exhausted:
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
        };
        let name = syn::LitStr::new(
            unit_ident.to_string().as_str(),
            Span::call_site(),
        );
        Ok(UnitDef {
            unit_ident,
            name,
            symbol,
            si_prefix,
            scale,
        })
    }
}

fn ref_unit_def_from_attr(ref_unit_attr: &syn::Attribute) -> UnitDef {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "2 or 3 comma-separated args expected.";
    const WRONG_TYPE_OF_ARG_ERROR: &str = "No scale expected for ref_unit.";
    const HELP: &str =
        "Use `#[ref_unit(<ident>, \"<symbol>\", <si_prefix>)]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\")]`.";

    match ref_unit_attr.parse_args::<UnitDef>() {
        Ok(mut unit_def) => {
            if unit_def.scale.is_some() {
                if unit_def.si_prefix.is_some() {
                    abort!(ref_unit_attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
                } else {
                    abort!(ref_unit_attr, WRONG_TYPE_OF_ARG_ERROR; help = HELP);
                }
            }
            unit_def.scale = Some(syn::LitFloat::new("1.0", Span::call_site()));
            unit_def
        }
        Err(_) => {
            abort!(ref_unit_attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
        }
    }
}

fn unit_defs_with_scale_from_attrs(
    attrs: &Vec<syn::Attribute>,
) -> Vec<UnitDef> {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "3 or 4 comma-separated args expected.";
    const NO_SCALE_ERROR: &str = "<scale> arg expected.";
    const HELP: &str =
        "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
         or  `#[unit(<ident>, \"<symbol>\", <scale>)]`.";

    let mut unit_defs: Vec<UnitDef> = vec![];
    for attr in attrs {
        match attr.parse_args::<UnitDef>() {
            Ok(unit_def) => {
                if unit_def.scale.is_none() {
                    abort!(attr, NO_SCALE_ERROR; help = HELP);
                }
                unit_defs.push(unit_def);
            }
            Err(_) => {
                abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
            }
        }
    }
    unit_defs
}

fn unit_defs_without_scale_from_attrs(
    attrs: &Vec<syn::Attribute>,
) -> Vec<UnitDef> {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str = "2 comma-separated args expected.";
    const HELP: &str = "Use `#[unit(<ident>, \"<symbol>\"]`.";

    let mut unit_defs: Vec<UnitDef> = vec![];
    for attr in attrs {
        match attr.parse_args::<UnitDef>() {
            Ok(unit_def) => {
                if unit_def.scale.is_some() || unit_def.si_prefix.is_some() {
                    abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
                }
                unit_defs.push(unit_def);
            }
            Err(_) => {
                abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
            }
        }
    }
    unit_defs
}

pub(crate) fn analyze(ast: &mut Ast) -> QtyDef {
    check_struct(ast);
    let attrs = &mut ast.attrs;
    let (unit_attrs, opt_ref_unit_attr) = get_unit_attrs(&attrs);
    attrs.retain(|attr| !(is_unit_attr(attr) || is_ref_unit_attr(attr)));
    let mut qty_def = QtyDef::new(ast.ident.clone());
    if let Some(ref_unit_attr) = opt_ref_unit_attr {
        let ref_unit_def = ref_unit_def_from_attr(&ref_unit_attr);
        qty_def.ref_unit_ident = Some(ref_unit_def.unit_ident.clone());
        qty_def.units = unit_defs_with_scale_from_attrs(&unit_attrs);
        qty_def.units.push(ref_unit_def);
    } else {
        qty_def.units = unit_defs_without_scale_from_attrs(&unit_attrs);
    }
    qty_def
}

fn codegen_attrs(attrs: &Vec<syn::Attribute>) -> TokenStream {
    let code = TokenStream::new();
    for attr in attrs {
        quote!(
            #code
            #attr
        );
    }
    code
}

fn codegen_unit_constants(
    enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let const_ident = syn::Ident::new(
            unit_ident.to_string().to_case(Case::UpperSnake).as_str(),
            Span::call_site(),
        );
        code = quote!(
            #code
            pub const #const_ident: #enum_ident = #enum_ident::#unit_ident;
        )
    }
    code
}

fn codegen_impl_mul_amnt_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Mul<#unit_enum_ident> for AmountT {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: #unit_enum_ident) -> Self::Output {
                #qty_ident::new(self, rhs)
            }
        }
        impl Mul<AmountT> for #unit_enum_ident {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: AmountT) -> Self::Output {
                #qty_ident::new(rhs, self)
            }
        }
    )
}

fn codegen_qty_single_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    unit_ident: &syn::Ident,
    unit_name: &syn::LitStr,
    unit_symbol: &syn::LitStr,
) -> TokenStream {
    quote!(
        pub type #qty_ident = Qty<#unit_enum_ident>;
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum #unit_enum_ident {
            #unit_ident,
        }
        impl Unit for #unit_enum_ident {
            const REF_UNIT: Option<Self> = None;
            fn name(&self) -> &'static str { #unit_name }
            fn symbol(&self) -> &'static str { #unit_symbol }
            fn si_prefix(&self) -> Option<SIPrefix> { None }
            fn scale(&self) -> Option<AmountT> { None }
        }
    )
}

fn codegen_unit_variants(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        code = quote!(
            #code
            #unit_ident,
        )
    }
    code
}

fn codegen_fn_name(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_name = unit.name.clone();
        code = quote!(
            #code
            #unit_enum_ident::#unit_ident => #unit_name,
        )
    }
    quote!(
        fn name(&self) -> &'static str {
            match self {
                #code
            }
        }
    )
}

fn codegen_fn_symbol(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_symbol = unit.symbol.clone();
        code = quote!(
            #code
            #unit_enum_ident::#unit_ident => #unit_symbol,
        )
    }
    quote!(
        fn symbol(&self) -> &'static str {
            match self {
                #code
            }
        }
    )
}

fn codegen_qty_without_ref_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let code_unit_variants = codegen_unit_variants(units);
    let code_fn_name = codegen_fn_name(unit_enum_ident, units);
    let code_fn_symbol = codegen_fn_symbol(unit_enum_ident, units);
    quote!(
        pub type #qty_ident = Qty<#unit_enum_ident>;
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum #unit_enum_ident { #code_unit_variants }
        impl Unit for #unit_enum_ident {
            const REF_UNIT: Option<Self> = None;
            #code_fn_name
            #code_fn_symbol
            fn si_prefix(&self) -> Option<SIPrefix> { None }
            fn scale(&self) -> Option<AmountT> { None }
        }
    )
}

fn codegen_fn_si_prefix(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.si_prefix.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_si_prefix: &syn::Ident = unit.si_prefix.as_ref().unwrap();
            code = quote!(
                #code
                #unit_enum_ident::#unit_ident => #unit_si_prefix,
            )
        }
    }
    quote!(
        fn si_prefix(&self) -> Option<SIPrefix> {
            match self {
                #code
                _ => None,
            }
        }
    )
}

fn codegen_fn_scale(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.scale.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_scale: &syn::LitFloat = unit.scale.as_ref().unwrap();
            code = quote!(
                #code
                #unit_enum_ident::#unit_ident => Some(Amnt!(#unit_scale)),
            )
        } else {
            // should not happen!
            abort_call_site!("Missing scale detected!")
        }
    }
    quote!(
        fn scale(&self) -> Option<AmountT> {
            match self {
                #code
            }
        }
    )
}

fn codegen_qty_with_ref_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    ref_unit_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let code_unit_variants = codegen_unit_variants(units);
    let code_fn_name = codegen_fn_name(unit_enum_ident, units);
    let code_fn_symbol = codegen_fn_symbol(unit_enum_ident, units);
    let code_fn_si_prefix = codegen_fn_si_prefix(unit_enum_ident, units);
    let code_fn_scale = codegen_fn_scale(unit_enum_ident, units);
    quote!(
        pub type #qty_ident = Qty<#unit_enum_ident>;
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum #unit_enum_ident {
            #code_unit_variants
        }
        impl Unit for #unit_enum_ident {
            const REF_UNIT: Option<Self> =
                Some(#unit_enum_ident::#ref_unit_ident);
            #code_fn_name
            #code_fn_symbol
            #code_fn_si_prefix
            #code_fn_scale
        }
    )
}

pub(crate) fn codegen(
    qty_def: &QtyDef,
    attrs: &Vec<syn::Attribute>,
) -> TokenStream {
    let qty_ident = qty_def.qty_ident.clone();
    let unit_enum_ident =
        syn::Ident::new(&*format!("{}Unit", qty_ident), Span::call_site());
    let code_attrs = codegen_attrs(attrs);
    let code_unit_consts =
        codegen_unit_constants(&unit_enum_ident, &qty_def.units);
    let code_impl_mul =
        codegen_impl_mul_amnt_unit(&qty_ident, &unit_enum_ident);
    let code_qty = if qty_def.units.len() == 1 {
        let unit_ident = qty_def.units[0].unit_ident.clone();
        let unit_name = qty_def.units[0].name.clone();
        let unit_symbol = qty_def.units[0].symbol.clone();
        codegen_qty_single_unit(
            &qty_ident,
            &unit_enum_ident,
            &unit_ident,
            &unit_name,
            &unit_symbol,
        )
    } else if qty_def.ref_unit_ident.is_none() {
        codegen_qty_without_ref_unit(
            &qty_ident,
            &unit_enum_ident,
            &qty_def.units,
        )
    } else {
        let ref_unit_ident: &syn::Ident =
            qty_def.ref_unit_ident.as_ref().unwrap();
        codegen_qty_with_ref_unit(
            &qty_ident,
            &unit_enum_ident,
            &ref_unit_ident,
            &qty_def.units,
        )
    };
    quote!(
        #code_attrs
        #code_qty
        #code_impl_mul
        #code_unit_consts
    )
}

#[cfg(test)]
mod internal_fn_tests {
    use super::*;

    fn get_ast() -> Ast {
        let args = quote!();
        let item = quote!(
            #[ref_unit(Megapop, "Mp", Mega)]
            #[unit(Pop, "p", 0.000001)]
            /// Quantity Foo
            struct Foo {}
        );
        parse(args, item)
    }

    #[test]
    fn test_parse() {
        let ast = get_ast();
        assert_eq!(ast.ident.to_string(), "Foo");
        assert!(ast.fields.is_empty());
        assert_eq!(ast.attrs.len(), 3);
        let attr_names: Vec<String> = ast
            .attrs
            .iter()
            .map(|attr| attr.path.segments.first().unwrap().ident.to_string())
            .collect();
        assert_eq!(attr_names, ["ref_unit", "unit", "doc"]);
    }

    #[test]
    fn test_analyze() {
        let mut ast = get_ast();
        let qty_def = analyze(&mut ast);
        assert_eq!(ast.attrs.len(), 1);
        assert_eq!(
            ast.attrs
                .first()
                .unwrap()
                .path
                .segments
                .first()
                .unwrap()
                .ident
                .to_string(),
            "doc"
        );
        assert_eq!(qty_def.qty_ident.to_string(), "Foo");
        assert_eq!(qty_def.ref_unit_ident.unwrap().to_string(), "Megapop");
        assert_eq!(qty_def.units.len(), 2);
        let unit = &qty_def.units[0];
        assert_eq!(unit.unit_ident.to_string(), "Pop");
        assert_eq!(unit.name.value(), "Pop");
        assert_eq!(unit.symbol.value(), "p");
        assert!(unit.si_prefix.is_none());
        assert_eq!(unit.scale.as_ref().unwrap().base10_digits(), "0.000001");
        let unit = &qty_def.units[1];
        assert_eq!(unit.unit_ident.to_string(), "Megapop");
        assert_eq!(unit.name.value(), "Megapop");
        assert_eq!(unit.symbol.value(), "Mp");
        assert_eq!(unit.si_prefix.as_ref().unwrap().to_string(), "Mega");
        assert_eq!(unit.scale.as_ref().unwrap().base10_digits(), "1.0");
    }
}
