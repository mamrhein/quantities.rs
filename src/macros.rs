// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(fpdec)]
#[allow(non_snake_case)]
#[macro_export]
macro_rules! Amnt {
    ($lit:literal) => {
        Dec!($lit)
    };
}
#[cfg(not(fpdec))]
#[allow(non_snake_case)]
#[macro_export]
macro_rules! Amnt {
    ($lit:literal) => {
        $lit
    };
}

#[macro_export]
macro_rules! opt {
    ($t:ty, $expr:expr) => {
        Option::<$t>::from($expr)
    };
}

#[macro_export]
macro_rules! impl_mul_amnt_unit {
    ($qty_id:ident, $unit_id:ident) => {
        impl Mul<$unit_id> for AmountT {
            type Output = $qty_id;
            #[inline(always)]
            fn mul(self, rhs: $unit_id) -> Self::Output {
                $qty_id::new(self, rhs)
            }
        }
        impl Mul<AmountT> for $unit_id {
            type Output = $qty_id;
            #[inline(always)]
            fn mul(self, rhs: AmountT) -> Self::Output {
                $qty_id::new(rhs, self)
            }
        }
    };
}

#[macro_export]
macro_rules! define_qty {
    // Quantity with single unit
    ($qty_id:ident,
     $unit_id:ident,
     $variant_id:ident,
     $variant_name:literal,
     $variant_symbol:literal) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum $unit_id {
            $variant_id,
        }
        pub const $variant_id: $unit_id = $unit_id::$variant_id;
        impl Unit for $unit_id {
            const REF_UNIT: Option<Self> = None;
            fn name(&self) -> &'static str { $variant_name }
            fn symbol(&self) -> &'static str { $variant_symbol }
            fn si_prefix(&self) -> Option<SIPrefix> { None }
            fn scale(&self) -> Option<AmountT> { None }
        }
        pub type $qty_id = Qty<$unit_id>;
        impl_mul_amnt_unit!($qty_id, $unit_id);
        };
    // Quantity w/o reference unit
    ($qty_id:ident,
     $unit_id:ident,
     $((
        $variant_id:ident,
        $variant_name:literal,
        $variant_symbol:literal
     )),+
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum $unit_id {
            $( $variant_id, )+
        }
        $( pub const $variant_id: $unit_id = $unit_id::$variant_id; )+
        impl Unit for $unit_id {
            const REF_UNIT: Option<Self> = None;
            fn name(&self) -> &'static str {
                match self {
                    $( $unit_id::$variant_id => $variant_name, )+
                }
            }
            fn symbol(&self) -> &'static str {
                match self {
                    $( $unit_id::$variant_id => $variant_symbol, )+
                }
            }
            fn si_prefix(&self) -> Option<SIPrefix> { None }
            fn scale(&self) -> Option<AmountT> { None }
        }
        pub type $qty_id = Qty<$unit_id>;
        impl_mul_amnt_unit!($qty_id, $unit_id);
    };
    // Quantity with reference unit
    ($qty_id:ident,
     $unit_id:ident,
     $ref_unit:ident,
     $((
        $variant_id:ident,
        $variant_name:literal,
        $variant_symbol:literal,
        $variant_si_prefix:ident,
        $variant_scale:literal
    )),+) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum $unit_id {
            $( $variant_id, )+
        }
        $( pub const $variant_id: $unit_id = $unit_id::$variant_id; )+
        impl Unit for $unit_id {
            const REF_UNIT: Option<Self> = Some($unit_id::$ref_unit);
            fn name(&self) -> &'static str {
                match self {
                    $( $unit_id::$variant_id => $variant_name, )+
                }
            }
            fn symbol(&self) -> &'static str {
                match self {
                    $( $unit_id::$variant_id => $variant_symbol, )+
                }
            }
            fn si_prefix(&self) -> Option<SIPrefix> {
                match self {
                    $( $unit_id::$variant_id =>
                        opt!(SIPrefix, $variant_si_prefix), )+
                }
            }
            fn scale(&self) -> Option<AmountT> {
                match self {
                    $( $unit_id::$variant_id => Some(Amnt!($variant_scale)), )+
                }
            }
        }
        pub type $qty_id = Qty<$unit_id>;
        impl_mul_amnt_unit!($qty_id, $unit_id);
    };
}
