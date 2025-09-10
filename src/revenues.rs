use serde::Serialize;

use crate::{Cost, costs::CostPeriods};

/// Grid benefit
/// A Swedish concept for "thanking" micro producers (<=43,5 kW) for reducing losses in the grid
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "value")]
pub enum FeedInRevenue {
    Simple(Cost),
    /// Not yet checked
    Unverified,
    /// Could not be located on their website or elsewhere
    Unlisted,
    /// Varies by the current spot price
    SpotPriceVariable {
        base_cost: Cost,
        spot_price_multiplier: f64,
        /// If this is approximated from actual data, or if it's based on documented pricing
        approximated: bool,
    },
    Periods {
        #[serde(flatten)]
        periods: CostPeriods,
    },
}

impl FeedInRevenue {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub(super) const fn new_periods(periods: CostPeriods) -> Self {
        Self::Periods { periods }
    }

    pub(super) const fn fixed_subunit(subunit: f64) -> Self {
        Self::Simple(Cost::fixed_subunit(subunit))
    }
}
