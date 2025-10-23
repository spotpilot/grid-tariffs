use std::ops::RangeInclusive;

use chrono::{Datelike, Timelike};
use indexmap::map::raw_entry_v1::RawEntryBuilder;
use serde::Serialize;

use crate::Language;

// A definition of hours (inclusive)
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct Hours(u8, u8);

impl Hours {
    pub(super) const fn new(from: u8, to_inclusive: u8) -> Self {
        Self(from, to_inclusive)
    }

    pub(super) const fn from(&self) -> u8 {
        self.0
    }

    #[allow(clippy::wrong_self_convention)]
    pub(super) const fn to_inclusive(&self) -> u8 {
        self.1
    }

    pub(super) fn translate(&self, language: Language) -> String {
        match language {
            // TODO: am/pm?
            Language::En => format!("{}-{}", self.from(), self.to_inclusive()),
            Language::Sv => format!("Kl {}-{}", self.from(), self.to_inclusive()),
        }
    }

    const fn start(&self) -> u8 {
        self.0
    }

    const fn end(&self) -> u8 {
        self.1
    }

    pub(crate) fn matches(&self, timestamp: chrono::DateTime<chrono_tz::Tz>) -> bool {
        if self.start() <= self.end() {
            (self.start()..=self.end()).contains(&(timestamp.hour() as u8))
        } else {
            (self.start()..=23).contains(&(timestamp.hour() as u8))
                || (0..=self.end()).contains(&(timestamp.hour() as u8))
        }
    }
}

// The supported main fuse sizes
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct MainFuseSizes {
    from: u16,
    to: u16,
}

impl MainFuseSizes {
    pub(super) const fn new_range(from: u16, to: u16) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(u8)]
pub(super) enum Month {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}

impl Month {
    pub(super) const fn translate(&self, language: Language) -> &'static str {
        match language {
            Language::En => match self {
                Self::January => "January",
                Self::February => "February",
                Self::March => "March",
                Self::April => "April",
                Self::May => "May",
                Self::June => "June",
                Self::July => "July",
                Self::August => "August",
                Self::September => "September",
                Self::October => "October",
                Self::November => "November",
                Self::December => "December",
            },
            Language::Sv => match self {
                Self::January => "januari",
                Self::February => "februari",
                Self::March => "mars",
                Self::April => "april",
                Self::May => "maj",
                Self::June => "juni",
                Self::July => "juli",
                Self::August => "augusti",
                Self::September => "september",
                Self::October => "oktober",
                Self::November => "november",
                Self::December => "december",
            },
        }
    }

    pub(crate) fn matches(&self, timestamp: chrono::DateTime<chrono_tz::Tz>) -> bool {
        timestamp.month0() == *self as u32
    }
}

/// An inclusive range of months
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct Months(Month, Month);

impl Months {
    pub(super) const fn new(from: Month, to: Month) -> Self {
        Self(from, to)
    }

    pub(super) fn translate(&self, language: Language) -> String {
        format!(
            "{} - {}",
            self.0.translate(language),
            self.1.translate(language)
        )
    }

    pub(crate) fn matches(&self, timestamp: chrono::DateTime<chrono_tz::Tz>) -> bool {
        let start_month_index = self.0 as u32;
        let end_month_index = self.1 as u32;
        let month_index = &timestamp.month0();

        if start_month_index <= end_month_index {
            (start_month_index..=end_month_index).contains(month_index)
        } else {
            (start_month_index..=11).contains(month_index)
                || (0..=end_month_index).contains(month_index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono_tz::Europe::Stockholm;

    #[test]
    fn hours_matches_exact_start() {
        let hours = Hours::new(6, 22);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 6, 0, 0).unwrap();
        assert!(hours.matches(timestamp));
    }

    #[test]
    fn hours_matches_exact_end() {
        let hours = Hours::new(6, 22);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 22, 59, 59).unwrap();
        assert!(hours.matches(timestamp));
    }

    #[test]
    fn hours_matches_middle() {
        let hours = Hours::new(6, 22);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 14, 30, 0).unwrap();
        assert!(hours.matches(timestamp));
    }

    #[test]
    fn hours_does_not_match_before() {
        let hours = Hours::new(6, 22);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 5, 59, 59).unwrap();
        assert!(!hours.matches(timestamp));
    }

    #[test]
    fn hours_does_not_match_after() {
        let hours = Hours::new(6, 22);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 23, 0, 0).unwrap();
        assert!(!hours.matches(timestamp));
    }

    #[test]
    fn hours_matches_midnight_range() {
        let hours = Hours::new(0, 5);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 3, 0, 0).unwrap();
        assert!(hours.matches(timestamp));
    }

    #[test]
    fn hours_matches_all_day() {
        let hours = Hours::new(0, 23);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap();
        assert!(hours.matches(timestamp));
    }

    #[test]
    fn month_matches_january() {
        let month = Month::January;
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap();
        assert!(month.matches(timestamp));
    }

    #[test]
    fn month_matches_december() {
        let month = Month::December;
        let timestamp = Stockholm
            .with_ymd_and_hms(2025, 12, 31, 23, 59, 59)
            .unwrap();
        assert!(month.matches(timestamp));
    }

    #[test]
    fn month_does_not_match_wrong_month() {
        let month = Month::June;
        let timestamp = Stockholm.with_ymd_and_hms(2025, 7, 15, 12, 0, 0).unwrap();
        assert!(!month.matches(timestamp));
    }

    #[test]
    fn month_matches_first_day() {
        let month = Month::March;
        let timestamp = Stockholm.with_ymd_and_hms(2025, 3, 1, 0, 0, 0).unwrap();
        assert!(month.matches(timestamp));
    }

    #[test]
    fn month_matches_last_day() {
        let month = Month::February;
        let timestamp = Stockholm.with_ymd_and_hms(2025, 2, 28, 23, 59, 59).unwrap();
        assert!(month.matches(timestamp));
    }

    #[test]
    fn months_matches_winter_period_start() {
        let months = Months::new(Month::November, Month::March);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 11, 1, 0, 0, 0).unwrap();
        assert!(months.matches(timestamp));
    }

    #[test]
    fn months_matches_winter_period_end() {
        let months = Months::new(Month::November, Month::March);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 3, 31, 23, 59, 59).unwrap();
        assert!(months.matches(timestamp));
    }

    #[test]
    fn months_matches_winter_period_middle() {
        let months = Months::new(Month::November, Month::March);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap();
        assert!(months.matches(timestamp));
    }

    #[test]
    fn months_does_not_match_before_range() {
        let months = Months::new(Month::November, Month::March);
        let timestamp = Stockholm
            .with_ymd_and_hms(2025, 10, 31, 23, 59, 59)
            .unwrap();
        assert!(!months.matches(timestamp));
    }

    #[test]
    fn months_does_not_match_after_range() {
        let months = Months::new(Month::November, Month::March);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap();
        assert!(!months.matches(timestamp));
    }

    #[test]
    fn months_matches_summer_period() {
        let months = Months::new(Month::May, Month::September);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 7, 15, 12, 0, 0).unwrap();
        assert!(months.matches(timestamp));
    }

    #[test]
    fn months_matches_single_month_range() {
        let months = Months::new(Month::June, Month::June);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
        assert!(months.matches(timestamp));
    }

    #[test]
    fn months_matches_full_year() {
        let months = Months::new(Month::January, Month::December);
        let timestamp = Stockholm.with_ymd_and_hms(2025, 8, 15, 12, 0, 0).unwrap();
        assert!(months.matches(timestamp));
    }
}
