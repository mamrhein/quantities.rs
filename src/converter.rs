// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::{AmountT, Quantity};

/// Trait for quantity converters
pub trait Converter<Q: Quantity> {
    /// Returns `conv` where `conv` ≣ `qty` and `conv.unit()` is `to_unit`, or
    /// `None` if conversion is not possible.
    fn convert(self, qty: &Q, to_unit: Q::UnitType) -> Option<Q>;
}

/// A table defining the conversion between instances of quantity `Q` having
/// different units.
///
/// Each entry of the table is holding the following elements:
/// * from_unit: Q::UnitType,
/// * to_unit: Q::UnitType,
/// * factor: AmountT,
/// * offset: AmountT
/// defining the conversion
/// to_amount = from_amount * factor + offset
#[derive(Debug)]
pub struct ConversionTable<Q: Quantity, const N: usize> {
    /// Table of tuples (from_unit, to_unit, factor, offset), defining the
    /// conversion to_amount = from_amount * factor + offset
    pub mappings: [(Q::UnitType, Q::UnitType, AmountT, AmountT); N],
}

impl<Q: Quantity, const N: usize> Converter<Q> for ConversionTable<Q, N> {
    fn convert(self, qty: &Q, to_unit: Q::UnitType) -> Option<Q> {
        if (*qty).unit() == to_unit {
            return Some(*qty);
        }
        self.mappings.iter().find_map(|(from, to, factor, offset)| {
            (*from == (*qty).unit() && *to == to_unit)
                .then(|| Q::new(qty.amount() * factor + offset, to_unit))
        })
    }
}
