use serde::Serialize;

use crate::{
    Cost, Money,
    costs::{CostPeriods, CostPeriodsSimple},
};

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "value")]
pub enum TransferFee {
    /// Price was not listed on their website
    Unlisted,
    /// Transfer fee has not been verified by us
    Unverified,
    /// Fee does not change except possibly by fuse size
    Simple(Cost),
    /// Transfer fee that varies by the current spot price
    /// We have currently only observed that Växjö Energi uses this variant
    SpotPriceVariable {
        base_cost: Money,
        spot_price_multiplier: f64,
        approximated: bool,
    },
    Periods(CostPeriods),
}

impl TransferFee {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub const fn simple_cost(&self) -> Option<Cost> {
        match self {
            Self::Simple(cost) => Some(*cost),
            _ => None,
        }
    }

    pub fn simplified(&self, fuse_size: u16, yearly_consumption: u32) -> TransferFeeSimplified {
        TransferFeeSimplified::new(self, fuse_size, yearly_consumption)
    }

    pub(super) const fn new_periods(periods: CostPeriods) -> Self {
        Self::Periods(periods)
    }

    pub(super) const fn fixed(int: i64, fract: u8) -> Self {
        Self::Simple(Cost::fixed(int, fract))
    }

    pub(super) const fn fixed_subunit(subunit: f64) -> Self {
        Self::Simple(Cost::fixed_subunit(subunit))
    }

    pub(super) fn is_yearly_consumption_based(&self, fuse_size: u16) -> bool {
        match self {
            TransferFee::Unlisted
            | TransferFee::Unverified
            | TransferFee::SpotPriceVariable { .. } => false,
            TransferFee::Simple(cost) => cost.is_yearly_consumption_based(fuse_size),
            TransferFee::Periods(periods) => periods.is_yearly_consumption_based(fuse_size),
        }
    }
}

/// Like TransferFee, but with costs being simple Money objects
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "value")]
pub enum TransferFeeSimplified {
    /// Price was not listed on their website
    Unlisted,
    /// Transfer fee has not been verified by us
    Unverified,
    /// Fee does not change except possibly by fuse size
    Simple(Option<Money>),
    /// Transfer fee that varies by the current spot price
    /// We have currently only observed that Växjö Energi uses this variant
    SpotPriceVariable {
        base_cost: Money,
        spot_price_multiplier: f64,
        approximated: bool,
    },
    Periods(CostPeriodsSimple),
}

impl TransferFeeSimplified {
    fn new(fee: &TransferFee, fuse_size: u16, yearly_consumption: u32) -> Self {
        match *fee {
            TransferFee::Unlisted => TransferFeeSimplified::Unlisted,
            TransferFee::Unverified => TransferFeeSimplified::Unverified,
            TransferFee::Simple(cost) => {
                TransferFeeSimplified::Simple(cost.cost_for(fuse_size, yearly_consumption))
            }
            TransferFee::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
            } => TransferFeeSimplified::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
            },
            TransferFee::Periods(periods) => TransferFeeSimplified::Periods(
                CostPeriodsSimple::new(periods, fuse_size, yearly_consumption),
            ),
        }
    }
}
