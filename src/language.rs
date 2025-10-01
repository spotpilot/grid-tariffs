use core::fmt;
use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Money, SE_TAX_REDUCTIONS, SE_TAXES, Tax, TaxAppliedBy, TaxReduction, helpers::date,
    tax_reductions,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Language {
    En,
    Sv,
}

impl Language {
    pub const fn all() -> &'static [Self] {
        &[Language::En, Language::Sv]
    }
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Sv => "sv",
        }
    }
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "en" => Ok(Language::En),
            "sv" => Ok(Language::Sv),
            _ => Err("no such language"),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Language::En => "en",
            Language::Sv => "sv",
        })
    }
}
