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

pub(crate) struct DerivedAs {
    lhs_ident: syn::Ident,
    op: syn::BinOp,
    rhs_ident: syn::Ident,
}

pub(crate) struct UnitDef {
    unit_ident: syn::Ident,
    name: syn::LitStr,
    symbol: syn::LitStr,
    si_prefix: Option<syn::Ident>,
    scale: Option<syn::Lit>,
    doc: Option<syn::LitStr>,
}

pub(crate) struct QtyDef {
    pub(crate) qty_ident: syn::Ident,
    pub(crate) derived_as: Option<DerivedAs>,
    pub(crate) ref_unit_ident: Option<syn::Ident>,
    pub(crate) units: Vec<UnitDef>,
}

impl QtyDef {
    fn new(qty_id: syn::Ident) -> Self {
        Self {
            qty_ident: qty_id,
            derived_as: None,
            ref_unit_ident: None,
            units: vec![],
        }
    }
}

pub(crate) type Item = syn::ItemStruct;

#[inline]
fn get_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Path(expr) => expr.path.get_ident(),
        _ => None,
    }
}

pub(crate) fn parse_args(args: TokenStream) -> Option<DerivedAs> {
    const ARGS_ERROR: &str =
        "Unknown argument(s) given to attribute `quantity`.";
    const OPERATOR_ERROR: &str = "Binary expression with '*' or '/' expected.";
    const OPERAND_ERROR: &str = "Identifier expected.";
    #[rustfmt::skip]
    const ARGS_HELP: &str =
        "Use `#[quantity]`\n\
         or  `#[quantity(<lhs_ident> * <rhs_ident>]`\n\
         or  `#[quantity(<lhs_ident> / <rhs_ident>]`.";

    if args.is_empty() {
        None
    } else if let Ok(expr) = syn::parse2::<syn::Expr>(args) {
        match expr {
            syn::Expr::Binary(args) => match args.op {
                syn::BinOp::Mul(_) | syn::BinOp::Div(_) => {
                    let lhs = get_ident(args.left.as_ref());
                    let rhs = get_ident(args.right.as_ref());
                    if lhs.is_none() || rhs.is_none() {
                        abort!(args, OPERAND_ERROR; help = ARGS_HELP)
                    }
                    Some(DerivedAs {
                        lhs_ident: lhs.unwrap().clone(),
                        op: args.op,
                        rhs_ident: rhs.unwrap().clone(),
                    })
                }
                _ => abort!(args, OPERATOR_ERROR; help = ARGS_HELP),
            },
            _ => abort!(expr, ARGS_ERROR; help = ARGS_HELP),
        }
    } else {
        abort_call_site!(ARGS_ERROR; help = ARGS_HELP)
    }
}

pub(crate) fn parse_item(item: TokenStream) -> Item {
    #[rustfmt::skip]
    const ITEM_HELP: &str =
        "Use `#[quantity]\n\
              ...\n\
              struct <ident> {}`.";

    match syn::parse2::<Item>(item.clone()) {
        Ok(item) => item,
        Err(error) => abort!(item, error; help = ITEM_HELP),
    }
}

fn check_struct(ast: &Item) {
    const GENERICS_ERROR: &str =
        "Given struct must not have generic parameters.";
    const FIELDS_ERROR: &str = "Given struct must not have fields.";
    let help = format!("Use `struct {} {{}};`", ast.ident);

    if !ast.generics.params.is_empty() {
        abort!(ast.generics, GENERICS_ERROR; help = help.as_str());
    }
    if !ast.fields.is_empty() {
        abort!(ast.fields, FIELDS_ERROR; help = help.as_str());
    }
}

#[inline]
fn is_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path()
        .is_ident(&syn::Ident::new("unit", Span::call_site()))
}

#[inline]
fn is_ref_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path()
        .is_ident(&syn::Ident::new("ref_unit", Span::call_site()))
}

const ARGS_LIST_ERROR: &str =
    "A comma-separated list of 2 to 5 arguments expected.";

#[rustfmt::skip]
const UNIT_ATTR_HELP: &str =
    "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>, \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <scale>, \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\")]`.";

fn get_unit_attrs(
    attrs: &Vec<syn::Attribute>,
) -> (Vec<syn::Attribute>, Option<syn::Attribute>) {
    const MORE_THAN_ONE_REFUNIT_ATTR_ERROR: &str =
        "There can only be one `refunit` attribute.";
    const NO_UNIT_ATTR_ERROR: &str =
        "At least one unit description must be given via attribute `unit`.";

    let mut unit_attrs: Vec<syn::Attribute> = vec![];
    let mut opt_ref_unit_attr: Option<syn::Attribute> = None;
    for attr in attrs {
        if is_unit_attr(attr) {
            unit_attrs.push(attr.clone());
        } else if is_ref_unit_attr(attr) {
            if opt_ref_unit_attr.is_some() {
                abort!(attr, MORE_THAN_ONE_REFUNIT_ATTR_ERROR);
            }
            opt_ref_unit_attr = Some(attr.clone());
        }
    }
    if unit_attrs.is_empty() {
        abort_call_site!(NO_UNIT_ATTR_ERROR; help = UNIT_ATTR_HELP);
    }
    (unit_attrs, opt_ref_unit_attr)
}

impl syn::parse::Parse for UnitDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut unit_ident: syn::Ident = input.parse()?;
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
        let mut scale: Option<syn::Lit> = None;
        if input.peek(syn::LitFloat) || input.peek(syn::LitInt) {
            scale = Some(input.parse::<syn::Lit>()?);
            let opt_comma: Option<syn::Token![,]> = input.parse()?;
            if opt_comma.is_none() && !input.is_empty() {
                return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
            }
        };
        let mut doc: Option<syn::LitStr> = None;
        if input.peek(syn::LitStr) {
            doc = Some(input.parse::<syn::LitStr>()?);
        }
        // Check if input is exhausted:
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
        };
        let name = syn::LitStr::new(
            unit_ident.to_string().replace('_', " ").as_str(),
            Span::call_site(),
        );
        unit_ident = syn::Ident::new(
            unit_ident.to_string().to_case(Case::UpperCamel).as_str(),
            Span::call_site(),
        );
        Ok(UnitDef {
            unit_ident,
            name,
            symbol,
            si_prefix,
            scale,
            doc,
        })
    }
}

fn ref_unit_def_from_attr(ref_unit_attr: &syn::Attribute) -> UnitDef {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "2, 3 or 4 comma-separated args expected.";
    const WRONG_TYPE_OF_ARG_ERROR: &str = "No scale expected for ref_unit.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[ref_unit(<ident>, \"<symbol>\", <si_prefix>, \"<doc>\")]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\", <si_prefix>)]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\")]`.";

    match ref_unit_attr.parse_args::<UnitDef>() {
        Ok(mut unit_def) => {
            if unit_def.scale.is_some() {
                abort!(ref_unit_attr, WRONG_TYPE_OF_ARG_ERROR; help = HELP);
            }
            unit_def.scale = Some(syn::Lit::Float(syn::LitFloat::new(
                "1.0",
                Span::call_site(),
            )));
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
        "3, 4 or 5 comma-separated args expected.";
    const NO_SCALE_ERROR: &str = "<scale> arg expected.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>, \"<doc>\")]`
         or  `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
         or  `#[unit(<ident>, \"<symbol>\", <scale>, \"<doc>\")]`\n\
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
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "2 or 3 comma-separated args expected.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
         or  `#[unit(<ident>, \"<symbol>\")]`.";

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

#[inline]
pub(crate) fn opt_lit_to_f64(lit: &Option<syn::Lit>) -> f64 {
    match lit.as_ref().unwrap() {
        syn::Lit::Float(f) => f.base10_parse().unwrap(),
        syn::Lit::Int(i) => i.base10_parse().unwrap(),
        _ => abort!(lit, "Internal error: unexspected non-numeric literal."),
    }
}

pub(crate) fn analyze(item_ast: &mut Item) -> QtyDef {
    check_struct(item_ast);
    let attrs = &mut item_ast.attrs;
    let (unit_attrs, opt_ref_unit_attr) = get_unit_attrs(attrs);
    attrs.retain(|attr| !(is_unit_attr(attr) || is_ref_unit_attr(attr)));
    let mut qty_def = QtyDef::new(item_ast.ident.clone());
    if let Some(ref_unit_attr) = opt_ref_unit_attr {
        let ref_unit_def = ref_unit_def_from_attr(&ref_unit_attr);
        qty_def.ref_unit_ident = Some(ref_unit_def.unit_ident.clone());
        qty_def.units = unit_defs_with_scale_from_attrs(&unit_attrs);
        qty_def.units.insert(0, ref_unit_def);
        qty_def.units.sort_by(|a, b| {
            let x = opt_lit_to_f64(&a.scale);
            let y = opt_lit_to_f64(&b.scale);
            x.partial_cmp(&y).unwrap()
        });
    } else {
        qty_def.units = unit_defs_without_scale_from_attrs(&unit_attrs);
        qty_def
            .units
            .sort_by(|a, b| a.name.value().cmp(&b.name.value()));
    }
    qty_def
}

fn codegen_attrs(attrs: &Vec<syn::Attribute>) -> TokenStream {
    let mut code = TokenStream::new();
    for attr in attrs {
        code = quote!(
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
        match &unit.doc {
            None => {
                code = quote!(
                    #code
                    pub const #const_ident: #enum_ident =
                        #enum_ident::#unit_ident;
                )
            }
            Some(doc) => {
                let unit_doc = doc.value();
                code = quote!(
                    #code
                    #[doc = #unit_doc]
                    pub const #const_ident: #enum_ident =
                        #enum_ident::#unit_ident;
                )
            }
        };
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
                Self::Output::new(self, rhs)
            }
        }
        impl Mul<AmountT> for #unit_enum_ident {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: AmountT) -> Self::Output {
                Self::Output::new(rhs, self)
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
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    quote!(
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub enum #unit_enum_ident {
            #unit_ident,
        }
        impl #unit_enum_ident {
            const VARIANTS: [Self; 1] = [Self::#unit_ident];
        }
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            fn name(&self) -> String { #unit_name.to_string() }
            fn symbol(&self) -> String { #unit_symbol.to_string() }
            fn si_prefix(&self) -> Option<SIPrefix> { None }
        }
        #[derive(Copy, Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub struct #qty_ident {
            amount: AmountT
        }
        impl Quantity for #qty_ident {
            type UnitType = #unit_enum_ident;

            #[inline(always)]
            fn new(amount: AmountT, _unit: Self::UnitType) -> Self {
                Self { amount }
            }

            #[inline(always)]
            fn amount(&self) -> AmountT {
                self.amount
            }

            #[inline(always)]
            fn unit(&self) -> Self::UnitType {
                Self::UnitType::#unit_ident
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.amount() + rhs.amount(), self.unit())
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.amount() - rhs.amount(), self.unit())
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = AmountT;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                self.amount() / rhs.amount()
            }
        }
    )
}

fn codegen_unit_variants(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        match &unit.doc {
            None => {
                code = quote!(
                    #code
                    #unit_ident,
                )
            }
            Some(doc) => {
                let unit_doc = doc.value();
                code = quote!(
                    #code
                    #[doc = #unit_doc]
                    #unit_ident,
                )
            }
        };
    }
    code
}

fn codegen_unit_variants_array(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    let n_variants = units.len();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        code = quote!(
            #code
            Self::#unit_ident,
        );
    }
    code = quote!(
        impl #unit_enum_ident {
            const VARIANTS: [Self; #n_variants] = [#code];
        }
    );
    code
}

fn codegen_fn_name(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_name = unit.name.clone();
        code = quote!(
            #code
            Self::#unit_ident => #unit_name.to_string(),
        )
    }
    quote!(
        fn name(&self) -> String {
            match self {
                #code
            }
        }
    )
}

fn codegen_fn_symbol(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_symbol = unit.symbol.clone();
        code = quote!(
            #code
            Self::#unit_ident => #unit_symbol.to_string(),
        )
    }
    quote!(
        fn symbol(&self) -> String {
            match self {
                #code
            }
        }
    )
}

fn codegen_impl_unit_display(unit_enum_ident: &syn::Ident) -> TokenStream {
    quote!(
        impl fmt::Display for #unit_enum_ident {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <Self as Unit>::fmt(self, f)
            }
        }
    )
}

fn codegen_impl_quantity(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        #[derive(Copy, Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub struct #qty_ident {
            amount: AmountT,
            unit: #unit_enum_ident
        }
        impl Quantity for #qty_ident {
            type UnitType = #unit_enum_ident;
            #[inline(always)]
            fn new(amount: AmountT, unit: Self::UnitType) -> Self {
                Self { amount, unit }
            }
            #[inline(always)]
            fn amount(&self) -> AmountT {
                self.amount
            }
            #[inline(always)]
            fn unit(&self) -> Self::UnitType {
                self.unit
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
    let code_unit_variants_array =
        codegen_unit_variants_array(unit_enum_ident, units);
    let code_fn_name = codegen_fn_name(units);
    let code_fn_symbol = codegen_fn_symbol(units);
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    let code_impl_quantity = codegen_impl_quantity(qty_ident, unit_enum_ident);
    quote!(
        #code_impl_quantity
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub enum #unit_enum_ident { #code_unit_variants }
        #code_unit_variants_array
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            #code_fn_name
            #code_fn_symbol
            fn si_prefix(&self) -> Option<SIPrefix> { None }
        }
        impl Eq for #qty_ident {}
        impl PartialEq<Self> for #qty_ident {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                <Self as Quantity>::eq(self, other)
            }
        }
        impl PartialOrd for #qty_ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                <Self as Quantity>::partial_cmp(self, other)
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                <Self as Quantity>::add(self, rhs)
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                <Self as Quantity>::sub(self, rhs)
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = AmountT;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                <Self as Quantity>::div(self, rhs)
            }
        }
    )
}

fn codegen_fn_si_prefix(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.si_prefix.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_si_prefix: &syn::Ident = unit.si_prefix.as_ref().unwrap();
            code = quote!(
                #code
                Self::#unit_ident =>
                    Some(SIPrefix::#unit_si_prefix),
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

fn codegen_fn_scale(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.scale.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_scale: &syn::Lit = unit.scale.as_ref().unwrap();
            code = quote!(
                #code
                Self::#unit_ident => Amnt!(#unit_scale),
            )
        } else {
            // should not happen!
            abort_call_site!("Missing scale detected!")
        }
    }
    quote!(
        fn scale(&self) -> AmountT {
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
    let code_unit_variants_array =
        codegen_unit_variants_array(unit_enum_ident, units);
    let code_fn_name = codegen_fn_name(units);
    let code_fn_symbol = codegen_fn_symbol(units);
    let code_fn_si_prefix = codegen_fn_si_prefix(units);
    let code_fn_scale = codegen_fn_scale(units);
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    let code_impl_quantity = codegen_impl_quantity(qty_ident, unit_enum_ident);
    quote!(
        #code_impl_quantity
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub enum #unit_enum_ident {
            #code_unit_variants
        }
        #code_unit_variants_array
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            #code_fn_name
            #code_fn_symbol
            #code_fn_si_prefix
        }
        impl LinearScaledUnit for #unit_enum_ident {
            const REF_UNIT: Self = Self::#ref_unit_ident;
            #code_fn_scale
        }
        impl HasRefUnit for #qty_ident {
            const REF_UNIT: #unit_enum_ident =
                #unit_enum_ident::#ref_unit_ident;
        }
        impl Eq for #qty_ident {}
        impl PartialEq<Self> for #qty_ident {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                <Self as HasRefUnit>::eq(self, other)
            }
        }
        impl PartialOrd for #qty_ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                <Self as HasRefUnit>::partial_cmp(self, other)
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::add(self, rhs)
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::sub(self, rhs)
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = AmountT;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::div(self, rhs)
            }
        }
    )
}

fn codegen_impl_std_traits(qty_ident: &syn::Ident) -> TokenStream {
    quote!(
        impl fmt::Display for #qty_ident {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <Self as Quantity>::fmt(self, f)
            }
        }
        impl Mul<#qty_ident> for AmountT {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: #qty_ident) -> Self::Output {
                Self::Output::new(self * rhs.amount(), rhs.unit())
            }
        }
        impl Mul<AmountT> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: AmountT) -> Self::Output {
                Self::Output::new(self.amount() * rhs, self.unit())
            }
        }
        impl Div<AmountT> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: AmountT) -> Self::Output {
                Self::Output::new(self.amount() / rhs, self.unit())
            }
        }
        impl<TQ: Quantity> Mul<Rate<TQ, Self>> for #qty_ident {
            type Output = TQ;

            fn mul(self, rhs: Rate<TQ, Self>) -> Self::Output {
                let amnt: AmountT =
                    (self / rhs.per_unit().as_qty()) / rhs.per_unit_multiple();
                Self::Output::new(amnt * rhs.term_amount(), rhs.term_unit())
            }
        }
        impl<PQ: Quantity> Div<Rate<Self, PQ>> for #qty_ident {
            type Output = PQ;

            fn div(self, rhs: Rate<Self, PQ>) -> Self::Output {
                let amnt: AmountT =
                    (self / rhs.term_unit().as_qty()) / rhs.term_amount();
                Self::Output::new(
                    amnt * rhs.per_unit_multiple(),
                    rhs.per_unit()
                )
            }
        }
    )
}

fn codegen_impl_qty_sqared(
    res_qty_ident: &syn::Ident,
    qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Mul<Self> for #qty_ident
        where
            Self: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn mul(self, rhs: Self) -> Self::Output {
                let scale =
                    self.unit().scale() * rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.amount() * rhs.amount(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            self.amount() * rhs.amount() * scale
                        )
                }
            }
        }
        impl<'a> Mul<#qty_ident> for &'a #qty_ident
        where
            #qty_ident: Mul<#qty_ident>,
        {
            type Output = <#qty_ident as Mul<#qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: #qty_ident) -> Self::Output {
                Mul::mul(*self, rhs)
            }
        }
        impl Mul<&Self> for #qty_ident
        where
            Self: Mul<Self>,
        {
            type Output = <Self as Mul<Self>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &Self) -> Self::Output {
                Mul::mul(self, *rhs)
            }
        }
        impl Mul<Self> for &#qty_ident
        where
            #qty_ident: Mul<#qty_ident>,
        {
            type Output = <#qty_ident as Mul<#qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self::Output {
                Mul::mul(*self, *rhs)
            }
        }
    )
}

fn codegen_impl_qty_mul_qty(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Mul<#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: HasRefUnit,
            #rhs_qty_ident: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn mul(self, rhs: #rhs_qty_ident) -> Self::Output {
                let scale =
                    self.unit().scale() * rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.amount() * rhs.amount(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            self.amount() * rhs.amount() * scale
                        )
                }
            }
        }
        impl<'a> Mul<#rhs_qty_ident> for &'a #lhs_qty_ident
        where
            #lhs_qty_ident: Mul<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: #rhs_qty_ident) -> Self::Output {
                Mul::mul(*self, rhs)
            }
        }
        impl Mul<&#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: Mul<#rhs_qty_ident>,
        {
            type Output = <Self as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Mul::mul(self, *rhs)
            }
        }
        impl Mul<&#rhs_qty_ident> for &#lhs_qty_ident
        where
            #lhs_qty_ident: Mul<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Mul::mul(*self, *rhs)
            }
        }
    )
}

fn codegen_impl_mul_qties(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    if lhs_qty_ident == rhs_qty_ident {
        let code = codegen_impl_qty_sqared(res_qty_ident, lhs_qty_ident);
        quote!(
            #code
        )
    } else {
        let code_lr = codegen_impl_qty_mul_qty(
            res_qty_ident,
            lhs_qty_ident,
            rhs_qty_ident,
        );
        let code_rl = codegen_impl_qty_mul_qty(
            res_qty_ident,
            rhs_qty_ident,
            lhs_qty_ident,
        );
        quote!(
            #code_lr
            #code_rl
        )
    }
}

fn codegen_impl_div_qties(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Div<#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: HasRefUnit,
            #rhs_qty_ident: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn div(self, rhs: #rhs_qty_ident) -> Self::Output {
                let scale =
                    self.unit().scale() / rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.amount() / rhs.amount(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            (self.amount() / rhs.amount()) * scale
                        )
                }
            }
        }
        impl<'a> Div<#rhs_qty_ident> for &'a #lhs_qty_ident
        where
            #lhs_qty_ident: Div<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: #rhs_qty_ident) -> Self::Output {
                Div::div(*self, rhs)
            }
        }
        impl Div<&#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: Div<#rhs_qty_ident>,
        {
            type Output = <Self as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Div::div(self, *rhs)
            }
        }
        impl Div<&#rhs_qty_ident> for &#lhs_qty_ident
        where
            #lhs_qty_ident: Div<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Div::div(*self, *rhs)
            }
        }
    )
}

fn codegen_impl_mul_div_qties(
    qty_ident: &syn::Ident,
    derived_as: &Option<DerivedAs>,
) -> TokenStream {
    match derived_as {
        None => TokenStream::new(),
        Some(derived_as) => {
            let lhs_qty_ident = &derived_as.lhs_ident;
            let rhs_qty_ident = &derived_as.rhs_ident;
            match derived_as.op {
                syn::BinOp::Mul(_) => {
                    let code_impl_mul = codegen_impl_mul_qties(
                        qty_ident,
                        lhs_qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_res_div_rhs = codegen_impl_div_qties(
                        lhs_qty_ident,
                        qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_res_div_lhs =
                        if lhs_qty_ident == rhs_qty_ident {
                            TokenStream::new()
                        } else {
                            codegen_impl_div_qties(
                                rhs_qty_ident,
                                qty_ident,
                                lhs_qty_ident,
                            )
                        };
                    quote!(
                        #code_impl_mul
                        #code_impl_res_div_rhs
                        #code_impl_res_div_lhs
                    )
                }
                syn::BinOp::Div(_) => {
                    let code_impl_div = codegen_impl_div_qties(
                        qty_ident,
                        lhs_qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_mul_res = codegen_impl_mul_qties(
                        lhs_qty_ident,
                        qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_div_res = codegen_impl_div_qties(
                        rhs_qty_ident,
                        lhs_qty_ident,
                        qty_ident,
                    );
                    quote!(
                        #code_impl_div
                        #code_impl_mul_res
                        #code_impl_div_res
                    )
                }
                _ => {
                    // should not happen!
                    abort_call_site!("Internal error: wrong op detected!")
                }
            }
        }
    }
}

pub(crate) fn codegen(
    qty_def: &QtyDef,
    attrs: &Vec<syn::Attribute>,
) -> TokenStream {
    let qty_ident = qty_def.qty_ident.clone();
    let unit_enum_ident =
        syn::Ident::new(&*format!("{}Unit", qty_ident), Span::call_site());
    let code_attrs = codegen_attrs(attrs);
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
            ref_unit_ident,
            &qty_def.units,
        )
    };
    let code_unit_consts =
        codegen_unit_constants(&unit_enum_ident, &qty_def.units);
    let code_impl_mul =
        codegen_impl_mul_amnt_unit(&qty_ident, &unit_enum_ident);
    let code_impl_unit_display = codegen_impl_unit_display(&unit_enum_ident);
    let code_impl_std_traits = codegen_impl_std_traits(&qty_ident);
    let code_mul_div_base_qties =
        codegen_impl_mul_div_qties(&qty_ident, &qty_def.derived_as);
    quote!(
        #code_attrs
        #code_qty
        #code_unit_consts
        #code_impl_mul
        #code_impl_unit_display
        #code_impl_std_traits
        #code_mul_div_base_qties
    )
}

#[cfg(test)]
mod internal_fn_tests {
    use super::*;

    fn get_ast_basic_qty() -> Item {
        let item = quote!(
            #[ref_unit(Megapop, "Mp", MEGA, "1000000·p\nFoo's reference unit")]
            #[unit(Gigapop, "Gp", GIGA, 1000, "1000000000·p")]
            #[unit(Pop, "p", 0.000001)]
            /// Quantity Foo
            struct Foo {}
        );
        parse_item(item)
    }

    #[test]
    fn test_parse_basic_qty() {
        let item = get_ast_basic_qty();
        assert_eq!(item.ident.to_string(), "Foo");
        assert!(item.fields.is_empty());
        assert_eq!(item.attrs.len(), 4);
        let attr_names: Vec<String> = item
            .attrs
            .iter()
            .map(|attr| attr.path().segments.first().unwrap().ident.to_string())
            .collect();
        assert_eq!(attr_names, ["ref_unit", "unit", "unit", "doc"]);
    }

    #[test]
    fn test_analyze_basic_qty() {
        let mut item = get_ast_basic_qty();
        let qty_def = analyze(&mut item);
        assert_eq!(item.attrs.len(), 1);
        assert_eq!(
            item.attrs
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
        assert_eq!(qty_def.units.len(), 3);
        let unit = &qty_def.units[0];
        assert_eq!(unit.unit_ident.to_string(), "Pop");
        assert_eq!(unit.name.value(), "Pop");
        assert_eq!(unit.symbol.value(), "p");
        assert!(unit.si_prefix.is_none());
        assert_eq!(opt_lit_to_f64(&unit.scale), 0.000001);
        assert!(unit.doc.is_none());
        let unit = &qty_def.units[1];
        assert_eq!(unit.unit_ident.to_string(), "Megapop");
        assert_eq!(unit.name.value(), "Megapop");
        assert_eq!(unit.symbol.value(), "Mp");
        assert_eq!(unit.si_prefix.as_ref().unwrap().to_string(), "MEGA");
        assert_eq!(opt_lit_to_f64(&unit.scale), 1.);
        assert_eq!(
            unit.doc.as_ref().unwrap().value(),
            "1000000·p\nFoo's reference unit"
        );
        let unit = &qty_def.units[2];
        assert_eq!(unit.unit_ident.to_string(), "Gigapop");
        assert_eq!(unit.name.value(), "Gigapop");
        assert_eq!(unit.symbol.value(), "Gp");
        assert_eq!(unit.si_prefix.as_ref().unwrap().to_string(), "GIGA");
        assert_eq!(opt_lit_to_f64(&unit.scale), 1000.);
        assert_eq!(unit.doc.as_ref().unwrap().value(), "1000000000·p");
    }

    #[test]
    fn test_codegen_remaining_attrs() {
        let mut item = get_ast_basic_qty();
        let _qty_def = analyze(&mut item);
        let code_attrs = codegen_attrs(&item.attrs);
        assert!(!code_attrs.is_empty());
        let doc = code_attrs.to_string();
        assert_eq!(doc, "# [doc = r\" Quantity Foo\"]");
    }

    #[test]
    fn test_codegen_unit_variants() {
        let mut item = get_ast_basic_qty();
        let qty_def = analyze(&mut item);
        let code_unit_variants = codegen_unit_variants(&qty_def.units);
        #[rustfmt::skip]
        assert_eq!(
            code_unit_variants.to_string(),
            "Pop , \
             # [doc = \"1000000·p\\nFoo's reference unit\"] Megapop , \
             # [doc = \"1000000000·p\"] Gigapop ,"
        );
    }

    fn get_ast_derived_qty() -> (Option<DerivedAs>, Item) {
        let args = quote!(Foo * Foo);
        let item = quote!(
            #[ref_unit(
                Megapop2,
                "Mp²",
                MEGA,
                "1000000·p²\nFooSquards's reference unit"
            )]
            #[unit(Gigapop2, "Gp²", GIGA, 1000, "1000000000·p")]
            #[unit(Pop2, "p²", 0.000001)]
            /// Quantity FooSquared
            struct FooSquared {}
        );
        (parse_args(args), parse_item(item))
    }

    #[test]
    fn test_parse_derived_qty() {
        let (opt_derived_as, item) = get_ast_derived_qty();
        assert!(opt_derived_as.is_some());
        let derived_as = opt_derived_as.unwrap();
        assert!(matches!(derived_as.op, syn::BinOp::Mul(_)));
        assert_eq!(derived_as.lhs_ident.to_string(), "Foo");
        assert_eq!(derived_as.rhs_ident.to_string(), "Foo");
        assert_eq!(item.ident.to_string(), "FooSquared");
        assert!(item.fields.is_empty());
        assert_eq!(item.attrs.len(), 4);
    }
}
