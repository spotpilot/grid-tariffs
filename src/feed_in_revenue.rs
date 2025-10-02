use serde::Serialize;

use crate::{Cost, CostPeriods, CostPeriodsSimple, Language, Money, currency::Currency};

/// Feed-in revenue, per kWh (usually from solar production)
/// A Swedish concept for "thanking" micro producers (<=43,5 kW) for reducing losses in the grid
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum FeedInRevenue {
    Simple(Cost),
    /// Not yet checked
    Unverified,
    /// Could not be located on their website or elsewhere
    Unlisted,
    /// Varies by the current spot price
    SpotPriceVariable {
        base_cost: Money,
        spot_price_multiplier: f64,
        /// If this is approximated from actual data, or if it's based on documented pricing
        approximated: bool,
    },
    Periods(CostPeriods),
}

impl FeedInRevenue {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub(super) const fn new_periods(periods: CostPeriods) -> Self {
        Self::Periods(periods)
    }

    pub(super) const fn fixed_subunit(subunit: f64) -> Self {
        Self::Simple(Cost::fixed_subunit(subunit))
    }

    pub fn simplified(
        &self,
        fuse_size: u16,
        yearly_consumption: u32,
        language: Language,
    ) -> FeedInRevenueSimplified {
        FeedInRevenueSimplified::new(self, fuse_size, yearly_consumption, language)
    }
}

/// Feed-in revenue, per kWh (usually from solar production)
/// Like FeedInRevenue, but with costs being simple Money objects
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum FeedInRevenueSimplified {
    Simple(Option<Money>),
    /// Not yet checked
    Unverified,
    /// Could not be located on their website or elsewhere
    Unlisted,
    /// Varies by the current spot price
    SpotPriceVariable {
        base_cost: Money,
        spot_price_multiplier: f64,
        /// If this is approximated from actual data, or if it's based on documented pricing
        approximated: bool,
        info: String,
    },
    Periods(CostPeriodsSimple),
}

impl FeedInRevenueSimplified {
    fn new(
        revenue: &FeedInRevenue,
        fuse_size: u16,
        yearly_consumption: u32,
        language: Language,
    ) -> Self {
        let revenue = match *revenue {
            FeedInRevenue::Unlisted => Self::Unlisted,
            FeedInRevenue::Unverified => Self::Unverified,
            FeedInRevenue::Simple(cost) => {
                Self::Simple(cost.cost_for(fuse_size, yearly_consumption))
            }
            FeedInRevenue::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
            } => Self::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
                info: Default::default(),
            },
            FeedInRevenue::Periods(periods) => Self::Periods(CostPeriodsSimple::new(
                periods,
                fuse_size,
                yearly_consumption,
                language,
            )),
        };
        revenue
    }

    fn add_info(mut self, language: Language) -> Self {
        match self {
            FeedInRevenueSimplified::SpotPriceVariable {
                base_cost,
                spot_price_multiplier,
                approximated,
                info,
            } => {
                let percentage = spot_price_multiplier * 100.;
                let mut info = match language {
                    Language::En => format!(
                        "The grid operator bases its feed-in revenue on a fixed part of {} and {}% of the current spot price.",
                        base_cost.display(Currency::SEK),
                        percentage
                    ),
                    Language::Sv => format!(
                        "Nätbolaget baserar sin nätnytta på en fast del om {} samt {}% av spotpriset.",
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
                FeedInRevenueSimplified::SpotPriceVariable {
                    base_cost,
                    spot_price_multiplier,
                    approximated,
                    info,
                }
            }
            _ => self,
        }
    }
}
