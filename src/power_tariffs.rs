use chrono::DateTime;
use chrono_tz::Tz;
use serde::Serialize;

use crate::{
    Language, Money,
    costs::{CostPeriods, CostPeriodsSimple, LoadType},
};

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PowerTariff {
    Unverified,
    NotImplemented,
    Periods {
        method: TariffCalculationMethod,
        periods: CostPeriods,
    },
}

impl PowerTariff {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub(super) const fn new(method: TariffCalculationMethod, periods: CostPeriods) -> Self {
        Self::Periods { method, periods }
    }

    pub(super) fn kw_cost(
        &self,
        timestamp: DateTime<Tz>,
        fuse_size: u16,
        yearly_consumption: u32,
    ) -> Money {
        let cost = Money::ZERO;
        match self {
            PowerTariff::Unverified | PowerTariff::NotImplemented => cost,
            PowerTariff::Periods { method, periods } => {
                for period in periods.iter() {
                    let money = period.cost().cost_for(fuse_size, yearly_consumption);
                }
                cost
            }
        }
    }

    pub fn simplified(
        &self,
        fuse_size: u16,
        yearly_consumption: u32,
        language: Language,
    ) -> PowerTariffSimplified {
        PowerTariffSimplified::new(self, fuse_size, yearly_consumption, language)
    }
}

/// The method used to calculate power tariffs
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum TariffCalculationMethod {
    /// Power peak for top hour of the top x days of the month
    AverageDays(u8),
    /// Average of top x hours of the month
    AverageHours(u8),
}

/// Like PowerTariff, but with costs being simple Money objects
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PowerTariffSimplified {
    Unverified,
    NotImplemented,
    Periods {
        method: TariffCalculationMethod,
        periods: CostPeriodsSimple,
    },
}

impl PowerTariffSimplified {
    fn new(fee: &PowerTariff, fuse_size: u16, yearly_consumption: u32, language: Language) -> Self {
        match *fee {
            PowerTariff::Unverified => PowerTariffSimplified::Unverified,
            PowerTariff::NotImplemented => PowerTariffSimplified::NotImplemented,
            PowerTariff::Periods { method, periods } => PowerTariffSimplified::Periods {
                method,
                periods: CostPeriodsSimple::new(periods, fuse_size, yearly_consumption, language),
            },
        }
    }
}
