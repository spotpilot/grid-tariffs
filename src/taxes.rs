use chrono::NaiveDate;
use serde::Serialize;

use crate::{Money, helpers::date};

pub(crate) static SE_TAXES: &'static [Tax] = &[
    Tax::new(
        "Energiskatt",
        Money::from_inner(54875),
        TaxAppliedBy::KwhConsumed,
        date(2025, 1, 1),
        Some(date(2025, 12, 31)),
    ),
    Tax::new(
        "Energiskatt",
        Money::from_inner(44995),
        TaxAppliedBy::KwhConsumed,
        date(2026, 1, 1),
        None,
    ),
];

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum TaxAppliedBy {
    KwhConsumed,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Tax {
    description: &'static str,
    cost: Money,
    applied_by: TaxAppliedBy,
    from_date: NaiveDate,
    /// Last date when this tax applies
    to_date: Option<NaiveDate>,
}

impl Tax {
    pub(crate) const fn new(
        description: &'static str,
        cost: Money,
        applied_by: TaxAppliedBy,
        from_date: NaiveDate,
        to_date: Option<NaiveDate>,
    ) -> Self {
        Self {
            description,
            cost,
            applied_by,
            from_date,
            to_date,
        }
    }

    pub(crate) fn valid_for(&self, today: NaiveDate) -> bool {
        today >= self.from_date && self.to_date.is_none_or(|to_date| today <= to_date)
    }
}
