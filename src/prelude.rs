// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! This module reexports all macros and types needed to define a quantity.

#[doc(hidden)]
pub use core::cmp::Ordering;
#[doc(hidden)]
pub use core::fmt;
#[doc(hidden)]
pub use core::ops::{Add, Div, Mul, Sub};

pub use qty_macros::quantity;

pub use crate::{
    Amnt, AmountT, HasRefUnit, LinearScaledUnit, Quantity, SIPrefix, Unit, ONE,
};
#[cfg(feature = "fpdec")]
pub use crate::{Dec, Decimal};
