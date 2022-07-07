// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use core::{
    fmt,
    ops::{Div, Mul},
};

use crate::{AmountT, Quantity, Unit, AMNT_ONE};

/// The ratio between two related quantity values.
#[derive(Copy, Clone, Debug)]
pub struct Rate<TQ: Quantity, PQ: Quantity> {
    term_amount: AmountT,
    term_unit: TQ::UnitType,
    per_unit_multiple: AmountT,
    per_unit: PQ::UnitType,
}

impl<TQ: Quantity, PQ: Quantity> Rate<TQ, PQ> {
    /// Returns a new instance of `Rate` with attributes equal to given params.
    #[inline(always)]
    pub const fn new(
        term_amount: AmountT,
        term_unit: TQ::UnitType,
        per_unit_multiple: AmountT,
        per_unit: PQ::UnitType,
    ) -> Self {
        Self {
            term_amount,
            term_unit,
            per_unit_multiple,
            per_unit,
        }
    }

    /// Returns a new instance of `Rate` with attributes extracted from the
    /// given quantity values.
    #[inline(always)]
    pub fn from_qty_vals(term: TQ, per: PQ) -> Self {
        Self {
            term_amount: term.amount(),
            term_unit: term.unit(),
            per_unit_multiple: per.amount(),
            per_unit: per.unit(),
        }
    }

    /// Returns the term amount of `self`.
    #[inline(always)]
    pub const fn term_amount(&self) -> AmountT {
        self.term_amount
    }

    /// Returns the term unit of `self`.
    #[inline(always)]
    pub const fn term_unit(&self) -> TQ::UnitType {
        self.term_unit
    }

    /// Returns the per unit multiple of `self`.
    #[inline(always)]
    pub const fn per_unit_multiple(&self) -> AmountT {
        self.per_unit_multiple
    }

    /// Returns the per unit of `self`.
    #[inline(always)]
    pub const fn per_unit(&self) -> PQ::UnitType {
        self.per_unit
    }

    /// Returns the multiplicative inverse of `self`
    pub const fn reciprocal(&self) -> Rate<PQ, TQ> {
        Rate::<PQ, TQ>::new(
            self.per_unit_multiple(),
            self.per_unit(),
            self.term_amount(),
            self.term_unit(),
        )
    }
}

impl<TQ: Quantity, PQ: Quantity> fmt::Display for Rate<TQ, PQ> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.term_unit().symbol() == "" {
            write!(f, "{} / ", self.term_amount())?;
        } else {
            write!(
                f,
                "{} {} / ",
                self.term_amount(),
                self.term_unit().symbol()
            )?;
        };
        if self.per_unit().symbol() == "" {
            write!(f, "{}", self.per_unit_multiple())
        } else if self.per_unit_multiple() == AMNT_ONE {
            write!(f, "{}", self.per_unit().symbol())
        } else {
            write!(
                f,
                "{} {}",
                self.per_unit_multiple(),
                self.per_unit().symbol()
            )
        }
    }
}

impl<TQ: Quantity, PQ: Quantity> Mul<PQ> for Rate<TQ, PQ>
where
    PQ: Div<PQ, Output = AmountT>,
{
    type Output = TQ;

    fn mul(self, rhs: PQ) -> Self::Output {
        let amnt: AmountT =
            (rhs / self.per_unit().as_qty()) / self.per_unit_multiple();
        Self::Output::new(amnt * self.term_amount(), self.term_unit())
    }
}
