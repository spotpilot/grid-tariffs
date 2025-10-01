use chrono::NaiveDate;
use serde::Serialize;

use crate::{Money, helpers::date};

pub(crate) static SE_TAX_REDUCTIONS: &[TaxReduction] = &[TaxReduction::new(
    "Skatteavdrag mikroproduktion",
    Money::from_inner(60000),
    TaxReductionAppliedBy::KwhFeedIn,
    date(2025, 1, 1),
    Some(date(2025, 12, 31)),
)];

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum TaxReductionAppliedBy {
    KwhFeedIn,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TaxReduction {
    description: &'static str,
    cost: Money,
    applied_by: TaxReductionAppliedBy,
    from_date: NaiveDate,
    /// Last date when this tax reduction applies
    to_date: Option<NaiveDate>,
}

impl TaxReduction {
    pub(crate) const fn new(
        description: &'static str,
        cost: Money,
        applied_by: TaxReductionAppliedBy,
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
