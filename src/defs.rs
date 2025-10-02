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
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
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
}
