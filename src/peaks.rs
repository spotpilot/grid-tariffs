use std::collections::HashMap;

use chrono::{DateTime, Datelike};
use chrono_tz::Tz;

use crate::{CostPeriod, CostPeriodMatching, CostPeriods, PowerTariff, TariffCalculationMethod};

#[derive(Clone, Debug)]
pub struct PeakPeriods {
    calc_method: TariffCalculationMethod,
    matching_method: CostPeriodMatching,
    items: Vec<PeriodDemand>,
}

impl PeakPeriods {
    pub fn new(
        calc_method: TariffCalculationMethod,
        periods: CostPeriods,
        mut averages: Vec<AverageDemand>,
    ) -> Self {
        let mut periods_averages_map = HashMap::new();

        for (p_idx, period) in periods.iter().enumerate() {
            let mut averages_for_period = vec![];

            for a_idx in (0..averages.len()).rev() {
                if period.matches(averages[a_idx].timestamp) {
                    averages_for_period.push(averages[a_idx].clone());

                    if periods.match_method() == CostPeriodMatching::First {
                        averages.remove(a_idx);
                    }
                }
            }

            periods_averages_map.insert(p_idx, averages_for_period);
        }

        let items = periods
            .clone()
            .iter()
            .enumerate()
            .map(|(p_idx, period)| PeriodDemand {
                period: period.clone(),
                peaks: PeakDemands::new(
                    calc_method,
                    period.clone(),
                    periods_averages_map.remove(&p_idx).unwrap(),
                ),
            })
            .collect();

        Self {
            calc_method,
            matching_method: periods.match_method(),
            items,
        }
    }

    pub fn items(&self) -> &[PeriodDemand] {
        &self.items
    }
}

#[derive(Clone, Debug)]
pub struct PeriodDemand {
    period: CostPeriod,
    peaks: PeakDemands,
}

impl PeriodDemand {
    pub fn period(&self) -> &CostPeriod {
        &self.period
    }
    pub fn peaks(&self) -> &PeakDemands {
        &self.peaks
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AverageDemand {
    pub timestamp: DateTime<Tz>,
    pub value: u32,
}

#[derive(Default, Clone, Debug)]
pub struct PeakDemands(Vec<AverageDemand>);

impl PeakDemands {
    pub fn new(
        calc_method: TariffCalculationMethod,
        period: CostPeriod,
        mut period_averages: Vec<AverageDemand>,
    ) -> Self {
        let peak_demands: Vec<AverageDemand> = match calc_method {
            crate::TariffCalculationMethod::AverageDays(n) => {
                // For AverageDays: get the highest hour from each of the top n days
                let mut daily_peaks: HashMap<chrono::NaiveDate, AverageDemand> = HashMap::new();

                // Group by day and keep only the highest value for each day
                for demand in period_averages.clone() {
                    let date = demand.timestamp.date_naive();
                    daily_peaks
                        .entry(date)
                        .and_modify(|existing| {
                            if demand.value > existing.value {
                                *existing = demand.clone();
                            }
                        })
                        .or_insert(demand);
                }

                // Convert to vector and sort by value descending
                let mut daily_peaks_vec: Vec<AverageDemand> = daily_peaks.into_values().collect();
                daily_peaks_vec.sort_by(|a, b| b.value.cmp(&a.value));

                // Take the top n days
                daily_peaks_vec.into_iter().take(n as usize).collect()
            }
            crate::TariffCalculationMethod::AverageHours(n) => {
                // For AverageHours: get the top n hours by value
                period_averages.sort_by(|a, b| b.value.cmp(&a.value));
                period_averages.into_iter().take(n as usize).collect()
            }
        };

        Self(peak_demands)
    }

    pub fn values(&self) -> &[AverageDemand] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Country, costs::LoadType, defs::Month};
    use chrono::{TimeZone, Timelike};
    use chrono_tz::Europe::Stockholm;

    fn create_simple_period() -> &'static CostPeriod {
        const PERIOD: CostPeriod = CostPeriod::builder()
            .load(LoadType::Low)
            .fixed_cost(5, 0)
            .build();
        &PERIOD
    }

    fn create_high_load_period() -> &'static CostPeriod {
        const PERIOD: CostPeriod = CostPeriod::builder()
            .load(LoadType::High)
            .fixed_cost(10, 0)
            .hours(6, 22)
            .months(Month::November, Month::March)
            .exclude_weekends()
            .exclude_holidays(Country::SE)
            .build();
        &PERIOD
    }

    #[test]
    fn average_hours_returns_n_highest_values() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                value: 100,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 1, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 2, 0, 0).unwrap(),
                value: 300,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 3, 0, 0).unwrap(),
                value: 200,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 4, 0, 0).unwrap(),
                value: 400,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageHours(3), period, averages);

        assert_eq!(peaks.values().len(), 3);
        assert_eq!(peaks.values()[0].value, 500);
        assert_eq!(peaks.values()[1].value, 400);
        assert_eq!(peaks.values()[2].value, 300);
    }

    #[test]
    fn average_hours_with_equal_values() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 1, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 2, 0, 0).unwrap(),
                value: 300,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageHours(2), period, averages);

        assert_eq!(peaks.values().len(), 2);
        assert_eq!(peaks.values()[0].value, 500);
        assert_eq!(peaks.values()[1].value, 500);
    }

    #[test]
    fn average_hours_empty_input() {
        let period = create_simple_period().clone();
        let averages = vec![];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageHours(3), period, averages);

        assert_eq!(peaks.values().len(), 0);
    }

    #[test]
    fn average_hours_zero_n() {
        let period = create_simple_period().clone();
        let averages = vec![AverageDemand {
            timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            value: 100,
        }];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageHours(0), period, averages);

        assert_eq!(peaks.values().len(), 0);
    }

    #[test]
    fn average_hours_n_greater_than_available() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
                value: 100,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 1, 0, 0).unwrap(),
                value: 200,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageHours(5), period, averages);

        assert_eq!(peaks.values().len(), 2);
        assert_eq!(peaks.values()[0].value, 200);
        assert_eq!(peaks.values()[1].value, 100);
    }

    #[test]
    fn average_days_one_peak_per_day() {
        let period = create_simple_period().clone();
        let averages = vec![
            // Day 1: peak at 500
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
                value: 100,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap(),
                value: 300,
            },
            // Day 2: peak at 600
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 2, 10, 0, 0).unwrap(),
                value: 200,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 2, 11, 0, 0).unwrap(),
                value: 600,
            },
            // Day 3: peak at 250
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 3, 10, 0, 0).unwrap(),
                value: 150,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 3, 11, 0, 0).unwrap(),
                value: 250,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(2), period, averages);

        assert_eq!(peaks.values().len(), 2);
        assert_eq!(peaks.values()[0].value, 600);
        assert_eq!(peaks.values()[0].timestamp.day(), 2);
        assert_eq!(peaks.values()[1].value, 500);
        assert_eq!(peaks.values()[1].timestamp.day(), 1);
    }

    #[test]
    fn average_days_ensures_different_days() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap(),
                value: 450,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 2, 10, 0, 0).unwrap(),
                value: 300,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(2), period, averages);

        assert_eq!(peaks.values().len(), 2);
        let day1 = peaks.values()[0].timestamp.date_naive();
        let day2 = peaks.values()[1].timestamp.date_naive();
        assert_ne!(day1, day2);
    }

    #[test]
    fn average_days_preserves_peak_hour_timestamp() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
                value: 100,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 14, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 20, 0, 0).unwrap(),
                value: 300,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(1), period, averages);

        assert_eq!(peaks.values().len(), 1);
        assert_eq!(peaks.values()[0].timestamp.hour(), 14);
        assert_eq!(peaks.values()[0].value, 500);
    }

    #[test]
    fn average_days_empty_input() {
        let period = create_simple_period().clone();
        let averages = vec![];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(3), period, averages);

        assert_eq!(peaks.values().len(), 0);
    }

    #[test]
    fn average_days_zero_n() {
        let period = create_simple_period().clone();
        let averages = vec![AverageDemand {
            timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            value: 100,
        }];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(0), period, averages);

        assert_eq!(peaks.values().len(), 0);
    }

    #[test]
    fn average_days_n_greater_than_available_days() {
        let period = create_simple_period().clone();
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
                value: 100,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 2, 10, 0, 0).unwrap(),
                value: 200,
            },
        ];

        let peaks = PeakDemands::new(TariffCalculationMethod::AverageDays(5), period, averages);

        assert_eq!(peaks.values().len(), 2);
    }

    #[test]
    fn peak_periods_first_matching_splits_values() {
        static PERIODS_ARRAY: [CostPeriod; 2] = [
            CostPeriod::builder()
                .load(LoadType::High)
                .fixed_cost(10, 0)
                .hours(6, 22)
                .months(Month::November, Month::March)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build(),
            CostPeriod::builder()
                .load(LoadType::Low)
                .fixed_cost(5, 0)
                .build(),
        ];
        let periods = CostPeriods::new_first(&PERIODS_ARRAY);

        // January 15, 2025 is a Wednesday
        let averages = vec![
            // Matches high: winter weekday 10:00
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap(),
                value: 500,
            },
            // Doesn't match high (hour 23): goes to low
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 15, 23, 0, 0).unwrap(),
                value: 300,
            },
            // Matches high: winter weekday 14:00
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 15, 14, 0, 0).unwrap(),
                value: 400,
            },
        ];

        let result = PeakPeriods::new(TariffCalculationMethod::AverageHours(10), periods, averages);

        assert_eq!(result.items().len(), 2);

        // High period gets 2 values (10:00, 14:00)
        assert_eq!(result.items()[0].peaks().values().len(), 2);
        assert_eq!(result.items()[0].peaks().values()[0].value, 500);
        assert_eq!(result.items()[0].peaks().values()[1].value, 400);

        // Low period gets 1 value (23:00)
        assert_eq!(result.items()[1].peaks().values().len(), 1);
        assert_eq!(result.items()[1].peaks().values()[0].value, 300);
    }

    #[test]
    fn peak_periods_all_matching_duplicates_values() {
        static PERIODS_ARRAY: [CostPeriod; 2] = [
            CostPeriod::builder()
                .load(LoadType::High)
                .fixed_cost(10, 0)
                .hours(6, 22)
                .months(Month::November, Month::March)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build(),
            CostPeriod::builder()
                .load(LoadType::Low)
                .fixed_cost(5, 0)
                .build(),
        ];
        let periods = CostPeriods::new_all(&PERIODS_ARRAY);

        // January 15, 2025 is a Wednesday
        let averages = vec![
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap(),
                value: 500,
            },
            AverageDemand {
                timestamp: Stockholm.with_ymd_and_hms(2025, 1, 15, 23, 0, 0).unwrap(),
                value: 300,
            },
        ];

        let result = PeakPeriods::new(TariffCalculationMethod::AverageHours(10), periods, averages);

        assert_eq!(result.items().len(), 2);

        // High period gets 1 value (only 10:00 matches criteria)
        assert_eq!(result.items()[0].peaks().values().len(), 1);
        assert_eq!(result.items()[0].peaks().values()[0].value, 500);

        // Low period gets both (no restrictions)
        assert_eq!(result.items()[1].peaks().values().len(), 2);
        assert_eq!(result.items()[1].peaks().values()[0].value, 500);
        assert_eq!(result.items()[1].peaks().values()[1].value, 300);
    }

    #[test]
    fn peak_periods_empty_averages() {
        static PERIODS_ARRAY: [CostPeriod; 1] = [CostPeriod::builder()
            .load(LoadType::Low)
            .fixed_cost(5, 0)
            .build()];
        let periods = CostPeriods::new_first(&PERIODS_ARRAY);
        let averages = vec![];

        let result = PeakPeriods::new(TariffCalculationMethod::AverageHours(3), periods, averages);

        assert_eq!(result.items().len(), 1);
        assert_eq!(result.items()[0].peaks().values().len(), 0);
    }

    #[test]
    fn single_value_both_methods() {
        let period = create_simple_period().clone();
        let averages = vec![AverageDemand {
            timestamp: Stockholm.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
            value: 100,
        }];

        let hours = PeakDemands::new(
            TariffCalculationMethod::AverageHours(3),
            period.clone(),
            averages.clone(),
        );
        assert_eq!(hours.values().len(), 1);
        assert_eq!(hours.values()[0].value, 100);

        let days = PeakDemands::new(TariffCalculationMethod::AverageDays(3), period, averages);
        assert_eq!(days.values().len(), 1);
        assert_eq!(days.values()[0].value, 100);
    }
}
