#![allow(unused)]
//! Grid operator information
//!
//! This information is written for consumers specifically
//! Costs for apartments are excluded as we do not aim to support those
//! All costs are specified with VAT included
//!
//! TODO: Change it so that a grid operator can have multiple price lists (e.g. Tekniska Verken becomes one)
//! TODO: Verify that we use the correct pricing and calculation method for each grid operator
//! TODO: Generate GridOperator entries from Tariff API
//!
use chrono::NaiveDate;

use crate::{
    builder::GridOperatorBuilder,
    costs::Cost,
    currency::Currency,
    defs::MainFuseSizes,
    fees::{OtherFees, TransferFee},
    money::Money,
    power_tariffs::PowerTariff,
    registry::sweden,
    revenues::FeedInRevenue,
};
pub use crate::{country::Country, links::*};

mod builder;
mod costs;
mod country;
mod currency;
mod defs;
mod fees;
mod helpers;
mod links;
mod money;
mod power_tariffs;
pub mod registry;
mod revenues;

#[derive(Debug, Clone)]
pub struct GridOperator {
    name: &'static str,
    vat_number: &'static str,
    price_date: NaiveDate,
    /// Costs are specified in this currency
    country: Country,
    /// The main fuse size range that this info covers
    main_fuses: MainFuseSizes,
    /// Fixed monthly fee
    monthly_fee: Cost,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Cost,
    transfer_fee: TransferFee,
    feed_in_revenue: FeedInRevenue,
    other_fees: OtherFees,
    power_tariff: PowerTariff,
    links: Links,
}

impl GridOperator {
    pub const fn name(&self) -> &str {
        &self.name
    }

    pub const fn vat_number(&self) -> &str {
        &self.vat_number
    }

    pub const fn price_date(&self) -> NaiveDate {
        self.price_date
    }

    pub const fn country(&self) -> Country {
        self.country
    }
    pub const fn monthly_fee(&self) -> &Cost {
        &self.monthly_fee
    }
    pub const fn monthly_production_fee(&self) -> &Cost {
        &self.monthly_production_fee
    }
    pub const fn transfer_fee(&self) -> &TransferFee {
        &self.transfer_fee
    }
    pub const fn feed_in_revenue(&self) -> &FeedInRevenue {
        &self.feed_in_revenue
    }
    pub const fn other_fees(&self) -> &OtherFees {
        &self.other_fees
    }
    pub const fn power_tariff(&self) -> &PowerTariff {
        &self.power_tariff
    }

    pub const fn links(&self) -> &Links {
        &self.links
    }

    pub const fn currency(&self) -> Currency {
        match self.country {
            Country::SE => Currency::SEK,
        }
    }

    pub fn get(country: Country, name: &str) -> Option<&'static Self> {
        match country {
            Country::SE => sweden::GRID_OPERATORS.iter().find(|o| o.name == name),
        }
    }

    pub fn where_name_starts_with(country: Country, text: &str) -> Vec<&'static Self> {
        match country {
            Country::SE => sweden::GRID_OPERATORS
                .iter()
                .filter(|go| {
                    go.name
                        .to_ascii_lowercase()
                        .starts_with(&text.to_ascii_lowercase())
                })
                .collect(),
        }
    }

    pub fn all() -> Vec<&'static Self> {
        sweden::GRID_OPERATORS.iter().collect()
    }

    pub(crate) const fn builder() -> GridOperatorBuilder {
        GridOperatorBuilder::new()
    }
}
