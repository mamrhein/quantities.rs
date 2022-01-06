// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! This module reexports all macros and types needed to define a quantity.

pub use crate::{Amnt, AmountT, Qty, Quantity, SIPrefix, Unit};
#[cfg(feature = "fpdec")]
pub use crate::{Dec, Decimal};
pub use core::ops::{Div, Mul};
pub use qty_macros::quantity;
