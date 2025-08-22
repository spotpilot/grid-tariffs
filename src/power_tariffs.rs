use chrono::DateTime;
use chrono_tz::Tz;

use crate::{
    costs::CostPeriods, money::Money
};

#[derive(Clone)]
pub struct PowerTariff {
    method: TariffCalculationMethod,
    periods: CostPeriods,
}

impl PowerTariff {
    pub(super) const fn new(method: TariffCalculationMethod, periods: CostPeriods) -> Self {
        Self { method, periods }
    }

    pub(super) fn kw_cost(
        &self,
        timestamp: DateTime<Tz>,
        fuse_size: u16,
        yearly_consumption: u32,
    ) -> Money {
        let cost = Money::default();
        for period in self.periods.iter() {
            let money = period
                .cost()
                .cost_for(fuse_size, yearly_consumption);
        }
        cost
    }
}

/// The method used to calculate power tariffs
#[derive(Clone, Copy)]
pub enum TariffCalculationMethod {
    /// Power peak for top hour of the top three days of the month
    AverageDays(u8),
    /// Average of top three hours of the month
    AverageHours(u8),
    /// Like AverageDays, but one for base load time and one for peak load time
    AverageDaysDifferentiated { peak: u8, base: u8 },
    /// Only count the max peak hour of the month
    PeakHour,
    /// Daytime and nighttime are calculated with different values
    AverageDayNightDifferentiated { day: i32, night: i32 },
}

impl TariffCalculationMethod {
    pub(super) fn relevant_samples(
        &self,
        grid_consumption: Vec<GridConsumption>,
    ) -> Vec<GridConsumption> {
        //     match self {
        //         TariffCalculationMethod::Unimplemented => vec![],
        //         TariffCalculationMethod::AverageDays(n) => grid_consumption
        //             .iter()
        //             .into_group_map_by(|pm| {
        //                 pm.timestamp()
        //                     .with_minute(0)
        //                     .with_second(0)
        //                     .with_nanosecond(0)
        //             })
        //             .into_iter()
        //             .map(|(_day_hour, pm)| pm.into_iter().max_by_key(|pm| pm.value()).unwrap())
        //             .copied()
        //             .take((*n).into())
        //             .collect_vec(),
        //         TariffCalculationMethod::AverageHours(n) => {
        //             grid_consumption.sort_by_key(|hp| hp.value());
        //             grid_consumption.into_iter().take((*n).into()).collect()
        //         }
        //         TariffCalculationMethod::AverageDaysDifferentiated { .. } => todo!(),
        //         TariffCalculationMethod::AverageDayNightDifferentiated { .. } => todo!(),
        //         TariffCalculationMethod::PeakHour => grid_consumption
        //             .iter()
        //             .max_by_key(|dw| dw.value())
        //             .map(|dw| vec![*dw])
        //             .unwrap_or_default(),
        //     }
        todo!()
    }
}

#[derive(Clone, Copy)]
pub(super) struct HourPower(DateTime<Tz>, u32);

impl HourPower {
    fn timestamp(&self) -> DateTime<Tz> {
        self.0
    }

    fn watts(&self) -> u32 {
        self.1
    }
}

// // Tekniska Verken Linköping, prislista alternativ
// // Skulle ge:
pub struct Peak {
    /// Non-inclusive time period
    time_period: (DateTime<Tz>, DateTime<Tz>),
    current_max_w: u32,
    cost_per_kw: Money,
    kw_divider: u8,
}

impl Peak {
    // pub fn kwh_cost(&self, _resolution: Resolution) -> Money {
    //     self.cost_per_kw
    // }

    pub fn contains(&self, timestamp: DateTime<Tz>) -> bool {
        timestamp >= self.time_period.0 && timestamp < self.time_period.1
    }
}

impl PowerTariff {
    pub fn get_peaks(
        &self,
        time_period: (DateTime<Tz>, DateTime<Tz>),
        grid_consumption: Vec<GridConsumption>,
    ) -> Vec<Peak> {
        let _samples = self.method.relevant_samples(grid_consumption);
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GridConsumption {
    timestamp: DateTime<Tz>,
    wh: u32,
}
