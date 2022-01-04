// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use quantities::prelude::*;

/// Foo, a completely useless quantity
#[quantity]
#[ref_unit(A, "a", MEGA)]
#[unit(B, "b", 0.4)]
#[unit(C, "c", 0.01)]
struct Foo<T> {}

fn main() {}
