use core::fmt;
use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Money, SE_TAX_REDUCTIONS, SE_TAXES, Tax, TaxAppliedBy, TaxReduction, helpers::date,
    tax_reductions,
};

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
            country: country,
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
