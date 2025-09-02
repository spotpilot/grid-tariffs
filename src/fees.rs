use serde::Serialize;

use crate::{Cost, costs::CostPeriods};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TransferFee {
    /// Price was not listed on their website
    Unlisted,
    /// Transfer fee has not been verified by us
    Unverified,
    /// Based on the time of day
    TimeOfDay {
        day: Cost,
        night: Cost,
    },
    /// Fee does not change except possibly by fuse size
    Simple(Cost),
    /// Transfer fee that varies by the current spot price
    /// We have currently only observed that Växjö Energi uses this variant
    SpotPriceVariable {
        base_cost: Cost,
        spot_price_multiplier: f64,
        approximated: bool,
    },
    Periods {
        periods: CostPeriods,
    },
}

impl TransferFee {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub(super) const fn new_periods(periods: CostPeriods) -> Self {
        Self::Periods { periods }
    }

    pub(super) const fn fixed(int: i64, fract: u8) -> Self {
        Self::Simple(Cost::fixed(int, fract))
    }

    pub(super) const fn fixed_subunit(subunit: f64) -> Self {
        Self::Simple(Cost::fixed_subunit(subunit))
    }
}

// Other kWh based fees
#[derive(Debug, Clone, Copy, Serialize)]
pub enum OtherFees {
    Unverified,
    List(&'static [(&'static str, Cost)]),
}

impl OtherFees {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }
}
