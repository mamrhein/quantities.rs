// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use quantities::prelude::*;

#[quantity]
#[ref_unit(Flop, "f")]
#[unit(Kiloflop, "kf", 1000., "1000·f")]
struct Foo {}

#[quantity]
#[unit(Emil, "e")]
struct Bar {}

#[quantity(Foo * Bar)]
#[ref_unit(Bazoo, "b", "1·f·e")]
#[unit(Millibazoo, "mb", 0.001, "0.001·b")]
struct Baz {}

fn main() {}
