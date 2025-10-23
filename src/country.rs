use core::fmt;
use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{Money, SE_TAX_REDUCTIONS, SE_TAXES, Tax, TaxAppliedBy, TaxReduction, helpers::date};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Country {
    SE,
}

impl Country {
    pub const fn all() -> &'static [Self] {
        &[Country::SE]
    }

    pub const fn taxes(&self) -> &'static [Tax] {
        match self {
            Country::SE => SE_TAXES,
        }
    }

    pub const fn tax_reductions(&self) -> &'static [TaxReduction] {
        match self {
            Country::SE => SE_TAX_REDUCTIONS,
        }
    }

    pub fn current_taxes(&self, today: NaiveDate) -> Vec<Tax> {
        self.taxes()
            .iter()
            .filter(|tax| tax.valid_for(today))
            .copied()
            .collect()
    }

    pub fn current_tax_reductions(&self, today: NaiveDate) -> Vec<TaxReduction> {
        self.tax_reductions()
            .iter()
            .filter(|tax| tax.valid_for(today))
            .copied()
            .collect()
    }

    pub(crate) const fn vat_rate(&self) -> f64 {
        match self {
            Country::SE => 1.25,
        }
    }

    pub(crate) const fn add_vat(&self, value: f64) -> f64 {
        value * self.vat_rate()
    }

    pub(crate) fn is_holiday(&self, date_naive: NaiveDate) -> bool {
        SE_HOLIDAYS.contains(&date_naive)
    }
}

impl Country {
    pub fn code(&self) -> &'static str {
        match self {
            Country::SE => "SE",
        }
    }

    pub fn english_name(&self) -> &'static str {
        match self {
            Country::SE => "Sweden",
        }
    }
}

impl FromStr for Country {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_ref() {
            "SE" => Ok(Country::SE),
            _ => Err("no such country"),
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Country::SE => "SE",
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CountryInfo {
    country: Country,
    /// Taxes applied in this country
    taxes: Vec<Tax>,
    /// Tax reductions applied in this country
    tax_reductions: Vec<TaxReduction>,
}

impl CountryInfo {
    pub fn current(country: Country) -> Self {
        let today = Utc::now().date_naive();
        Self {
            country,
            taxes: country.current_taxes(today),
            tax_reductions: country.current_tax_reductions(today),
        }
    }
}

impl From<Country> for CountryInfo {
    fn from(country: Country) -> Self {
        let taxes = country.taxes().to_vec();
        let tax_reductions = country.tax_reductions().to_vec();
        Self {
            country,
            taxes,
            tax_reductions,
        }
    }
}

static SE_HOLIDAYS: &[NaiveDate] = &[
    date(2025, 1, 1),
    date(2025, 1, 6),
    date(2025, 4, 18),
    date(2025, 4, 20),
    date(2025, 4, 21),
    date(2025, 5, 1),
    date(2025, 5, 29),
    date(2025, 6, 6),
    date(2025, 6, 8),
    date(2025, 6, 21),
    date(2025, 11, 1),
    date(2025, 12, 24),
    date(2025, 12, 25),
    date(2025, 12, 26),
    date(2025, 12, 31),
    date(2026, 1, 1),
    date(2026, 1, 6),
    date(2026, 4, 3),
    date(2026, 4, 5),
    date(2026, 4, 6),
    date(2026, 5, 1),
    date(2026, 5, 14),
    date(2026, 5, 24),
    date(2026, 6, 6),
    date(2026, 6, 20),
    date(2026, 10, 31),
    date(2026, 12, 24),
    date(2026, 12, 25),
    date(2026, 12, 26),
    date(2026, 12, 31),
    date(2027, 1, 1),
    date(2027, 1, 6),
    date(2027, 3, 26),
    date(2027, 3, 28),
    date(2027, 3, 29),
    date(2027, 5, 1),
    date(2027, 5, 6),
    date(2027, 5, 16),
    date(2027, 6, 6),
    date(2027, 6, 26),
    date(2027, 11, 6),
    date(2027, 12, 24),
    date(2027, 12, 25),
    date(2027, 12, 26),
    date(2027, 12, 31),
    date(2028, 1, 1),
    date(2028, 1, 6),
    date(2028, 4, 14),
    date(2028, 4, 16),
    date(2028, 4, 17),
    date(2028, 5, 1),
    date(2028, 5, 25),
    date(2028, 6, 4),
    date(2028, 6, 6),
    date(2028, 6, 24),
    date(2028, 11, 4),
    date(2028, 12, 24),
    date(2028, 12, 25),
    date(2028, 12, 26),
    date(2028, 12, 31),
];
