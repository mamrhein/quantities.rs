// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

//! Definition of derived quantity `Acceleration`.

use crate::duration::Duration;
use crate::prelude::*;
use crate::speed::Speed;

#[quantity(Speed / Duration)]
#[ref_unit(
    Meter_per_Second_squared,
    "m/s²",
    NONE,
    "Reference unit of quantity `Acceleration`"
)]
#[unit(Yards_per_Second_squared, "yd/s²", 0.9144, "yd/s²")]
/// Rate of change of an objects speed with respect to time.
///
/// Definition: Speed/Duration = Length/Duration²
///
/// Reference unit: Meter per Second squared ('m/s²')
///
/// Predefined units:
///
/// | Symbol | Name                     | Definition    | Equivalent in 'm/s²' |
/// |--------|--------------------------|---------------|----------------------|
/// | yd/s²  | Yards per Second squared | yd/s²         | 0.9144               |
pub struct Acceleration {}
