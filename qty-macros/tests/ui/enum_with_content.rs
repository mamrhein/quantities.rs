// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use qty_macros::VariantsAsConstants;

#[derive(VariantsAsConstants, Debug, PartialEq)]
pub enum TestEnum {
    A(i32),
    B,
}

fn main() {}
