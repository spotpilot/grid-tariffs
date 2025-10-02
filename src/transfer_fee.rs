use serde::Serialize;

use crate::{Cost, CostPeriods, CostPeriodsSimple, Language, Money, currency::Currency};

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

    pub const fn spot_price_variable(
        base_cost_subunit: f64,
        spot_price_multiplier: f64,
        approximated: bool,
    ) -> Self {
        Self::SpotPriceVariable {
            base_cost: Money::new_subunit(base_cost_subunit),
            spot_price_multiplier,
            approximated,
        }
    }

    pub fn simplified(
        &self,
        fuse_size: u16,
        yearly_consumption: u32,
        language: Language,
    ) -> TransferFeeSimplified {
        TransferFeeSimplified::new(self, fuse_size, yearly_consumption, language)
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

    /// Use when the operator states that they use spot price variable pricing, but don't specify the actual multipliers
    pub(crate) const fn spot_price_variable_placeholder() -> TransferFee {
        Self::SpotPriceVariable {
            base_cost: Money::new_subunit(2.0),
            spot_price_multiplier: 0.06,
            approximated: true,
        }
    }
}

/// Like TransferFee, but with costs being simple Money objects
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum TransferFeeSimplified {
    /// Price was not listed on their website
    Unlisted,
    /// Transfer fee has not been verified by us
    Unverified,
    /// Fee does not change except possibly by fuse size
    Simple(Money),
    /// Transfer fee that varies by the current spot price
    /// We have currently only observed that Växjö Energi uses this variant
    SpotPriceVariable {
        base_cost: Money,
        spot_price_multiplier: f64,
        approximated: bool,
        info: String,
    },
    Periods(CostPeriodsSimple),
}

impl TransferFeeSimplified {
    fn new(fee: &TransferFee, fuse_size: u16, yearly_consumption: u32, language: Language) -> Self {
        match *fee {
            TransferFee::Unlisted => TransferFeeSimplified::Unlisted,
            TransferFee::Unverified => TransferFeeSimplified::Unverified,
            TransferFee::Simple(cost) => TransferFeeSimplified::Simple(
                cost.cost_for(fuse_size, yearly_consumption)
                    .unwrap_or(Money::ZERO),
            ),
            TransferFee::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
            } => TransferFeeSimplified::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
                info: Default::default(),
            },
            TransferFee::Periods(periods) => TransferFeeSimplified::Periods(
                CostPeriodsSimple::new(periods, fuse_size, yearly_consumption, language),
            ),
        }
        .add_info(language)
    }

    fn add_info(mut self, language: Language) -> Self {
        match self {
            TransferFeeSimplified::Unlisted => self,
            TransferFeeSimplified::Unverified => self,
            TransferFeeSimplified::Simple(_) => self,
            TransferFeeSimplified::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
                info,
            } => {
                let percentage = spot_price_multiplier * 100.;
                let mut info = match language {
                    Language::En => format!(
                        "The grid operator bases its transfer fee on a fixed part of {} and {}% of the current spot price.",
                        base_cost.display(Currency::SEK),
                        percentage
                    ),
                    Language::Sv => format!(
                        "Nätbolaget baserar sin överföringsavgift på en fast del om {} samt {}% av spotpriset.",
                        base_cost.display(Currency::SEK),
                        percentage
                    ),
                };
                if approximated {
                    info.push_str(&match language {
                        Language::En => format!(
                            " The percentage is estimated as the grid operator doesn't list it on their website."
                        ),
                        Language::Sv => format!(
                            " Procentsatsen är uppskattad eftersom nätbolaget inte skriver ut exakt vad den är på sin webbplats."
                        ),
                    })
                }
                TransferFeeSimplified::SpotPriceVariable {
                    base_cost,
                    spot_price_multiplier,
                    approximated,
                    info,
                }
            }
            TransferFeeSimplified::Periods(_) => self,
        }
    }
}
