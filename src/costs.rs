use std::slice::Iter;

use chrono::DateTime;
use chrono_tz::Tz;
use serde::Serialize;

use crate::{
    Country,
    defs::{Hours, Month, Months},
    money::Money,
};

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Cost {
    None,
    /// Cost has not been verified
    Unverified,
    Fixed(Money),
    Fuses(&'static [(u16, Money)]),
    /// Fuse size combined with a yearly energy consumption limit
    FuseRangeYearlyConsumption(&'static [(u16, u16, Option<u32>, Money)]),
    FuseRange(&'static [(u16, u16, Money)]),
}

impl Cost {
    pub const fn is_unverified(&self) -> bool {
        matches!(self, Self::Unverified)
    }

    pub(super) const fn fuses(values: &'static [(u16, Money)]) -> Self {
        Self::Fuses(values)
    }
    pub(super) const fn fuse_range(ranges: &'static [(u16, u16, Money)]) -> Self {
        Self::FuseRange(ranges)
    }

    pub(super) const fn fuse_range_with_yearly_consumption(
        values: &'static [(u16, u16, Option<u32>, Money)],
    ) -> Cost {
        Self::FuseRangeYearlyConsumption(values)
    }

    pub(super) const fn fixed(int: i64, fract: u8) -> Self {
        Self::Fixed(Money::new(int, fract))
    }

    pub(super) const fn fixed_yearly(int: i64, fract: u8) -> Self {
        Self::Fixed(Money::new(int, fract).divide_by(12))
    }

    pub(super) const fn fixed_subunit(subunit: f64) -> Self {
        Self::Fixed(Money::new_subunit(subunit))
    }

    pub const fn cost_for(&self, fuse_size: u16, yearly_consumption: u32) -> Option<Money> {
        match *self {
            Cost::None => None,
            Cost::Unverified => None,
            Cost::Fixed(money) => Some(money),
            Cost::Fuses(values) => {
                let mut i = 0;
                while i < values.len() {
                    let (size, money) = values[i];
                    if fuse_size == size {
                        return Some(money);
                    }
                    i += 1;
                }
                None
            }
            Cost::FuseRangeYearlyConsumption(values) => {
                let mut i = 0;
                while i < values.len() {
                    let (min_size, max_size, max_consumption, money) = values[i];
                    if fuse_size >= min_size && fuse_size <= max_size {
                        if let Some(max_consumption) = max_consumption {
                            if max_consumption <= yearly_consumption {
                                return Some(money);
                            }
                        } else {
                            return Some(money);
                        }
                    }
                    i += 1;
                }
                None
            }
            Cost::FuseRange(ranges) => {
                let mut i = 0;
                while i < ranges.len() {
                    let (min, max, money) = ranges[i];
                    if fuse_size >= min && fuse_size <= max {
                        return Some(money);
                    }
                    i += 1;
                }
                None
            }
        }
    }

    pub(crate) const fn add_vat(&self, country: Country) -> Cost {
        let rate = match country {
            Country::SE => 1.25,
        };
        match self {
            Cost::None => Cost::None,
            Cost::Unverified => Cost::Unverified,
            Cost::Fixed(money) => Cost::Fixed(money.add_vat(country)),
            Cost::Fuses(items) => todo!(),
            Cost::FuseRangeYearlyConsumption(items) => todo!(),
            Cost::FuseRange(items) => todo!(),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CostPeriods {
    periods: &'static [CostPeriod],
}
impl CostPeriods {
    pub(super) const fn new(periods: &'static [CostPeriod]) -> Self {
        Self { periods }
    }

    pub(super) fn iter(&self) -> Iter<'_, CostPeriod> {
        self.periods.iter()
    }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct CostPeriod {
    cost: Cost,
    load: LoadType,
    include: [Option<PeriodType>; 2],
    exclude: [Option<PeriodType>; 2],
    /// Divide kw by this amount during this period
    divide_kw_by: u8,
}

impl CostPeriod {
    pub(super) const fn builder() -> CostPeriodBuilder {
        CostPeriodBuilder::new()
    }

    pub const fn cost(&self) -> Cost {
        self.cost
    }

    pub const fn load(&self) -> LoadType {
        self.load
    }

    pub fn matches(&self, _timestamp: DateTime<Tz>) -> bool {
        for _period_type in self.include_period_types() {
            // TODO: self-contain PeriodType, i.e. WinterNights becomes Months::new() + Hours::new()
            // period_type.matches(timestamp)
        }
        todo!()
    }

    fn include_period_types(&self) -> Vec<PeriodType> {
        self.include.iter().flatten().copied().collect()
    }

    fn exclude_period_types(&self) -> Vec<PeriodType> {
        self.exclude.iter().flatten().copied().collect()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum LoadType {
    /// Base load. Always counts
    Base,
    /// Low load period. Commonly counts during night hours and the summer half of the year
    Low,
    /// High load period. Commonly counts during daytime hours and the winter half of the year
    High,
}

pub(super) use LoadType::*;

#[derive(Clone)]
pub(super) struct CostPeriodBuilder {
    cost: Cost,
    load: Option<LoadType>,
    include: [Option<PeriodType>; 2],
    exclude: [Option<PeriodType>; 2],
    /// Divide kw by this amount during this period
    divide_kw_by: u8,
}

impl CostPeriodBuilder {
    pub(super) const fn new() -> Self {
        Self {
            cost: Cost::None,
            load: None,
            include: [None; 2],
            exclude: [None; 2],
            divide_kw_by: 1,
        }
    }

    pub(super) const fn build(self) -> CostPeriod {
        CostPeriod {
            cost: self.cost,
            load: self.load.expect("`load` must be specified"),
            include: self.include,
            exclude: self.exclude,
            divide_kw_by: self.divide_kw_by,
        }
    }

    pub(super) const fn cost(mut self, cost: Cost) -> Self {
        self.cost = cost;
        self
    }

    pub(super) const fn load(mut self, load: LoadType) -> Self {
        self.load = Some(load);
        self
    }

    pub(super) const fn fixed_cost(mut self, int: i64, fract: u8) -> Self {
        self.cost = Cost::fixed(int, fract);
        self
    }

    pub(super) const fn fixed_cost_subunit(mut self, subunit: f64) -> Self {
        self.cost = Cost::fixed_subunit(subunit);
        self
    }

    pub(super) const fn include(mut self, period_type: PeriodType) -> Self {
        let mut i = 0;
        while i < self.include.len() {
            if self.include[i].is_some() {
                i += 1;
            } else {
                self.include[i] = Some(period_type);
                return self;
            }
        }
        panic!("Too many includes");
    }

    pub(super) const fn months(self, from: Month, to: Month) -> Self {
        self.include(PeriodType::Months(Months::new(from, to)))
    }

    pub(super) const fn month(self, month: Month) -> Self {
        self.include(PeriodType::Month(month))
    }

    pub(super) const fn hours(self, from: u8, to_inclusive: u8) -> Self {
        self.include(PeriodType::Hours(Hours::new(from, to_inclusive)))
    }

    pub(super) const fn exclude(mut self, period_type: PeriodType) -> Self {
        let mut i = 0;
        while i < self.exclude.len() {
            if self.exclude[i].is_some() {
                i += 1;
            } else {
                self.exclude[i] = Some(period_type);
                return self;
            }
        }
        panic!("Too many excludes");
    }

    pub(super) const fn exclude_weekends_and_swedish_holidays(self) -> Self {
        self.exclude_weekends().exclude(PeriodType::SwedishHolidays)
    }

    pub(super) const fn exclude_weekends(self) -> Self {
        self.exclude(PeriodType::Weekends)
    }

    pub(super) const fn divide_kw_by(mut self, value: u8) -> Self {
        self.divide_kw_by = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Cost;
    use crate::money::Money;

    #[test]
    fn fuse_based_cost() {
        const FUSE_BASED: Cost = Cost::fuse_range(&[
            (16, 35, Money::new(54, 0)),
            (35, u16::MAX, Money::new(108, 50)),
        ]);
        assert_eq!(FUSE_BASED.cost_for(10, 0), None);
        assert_eq!(FUSE_BASED.cost_for(25, 0), Some(Money::new(54, 0)));
        assert_eq!(FUSE_BASED.cost_for(200, 0), Some(Money::new(108, 50)));
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) enum PeriodType {
    Months(Months),
    Month(Month),
    Hours(Hours),
    Weekends,
    SwedishHolidays,
}
