#![allow(unused)]
//! Grid operator information
//!
//! This information is written for consumers specifically
//! Costs for apartments are excluded as we do not aim to support those
//! All costs are specified with VAT included
//!
//! TODO: Verify that we use the correct pricing and calculation method for each grid operator
//! TODO: CostPeriod::fallthrough() is maybe not a very good concept when we need to calculate later on..?
//! TODO: Generate GridOperator entries from Tariff API
//! TODO: Add all known tariff API
//!
use chrono::NaiveDate;

use crate::{
    costs::Cost,
    country::Country,
    currency::Currency,
    defs::MainFuseSizes,
    fees::{OtherFees, TransferFee},
    links::Links,
    money::Money,
    power_tariffs::PowerTariff,
    registry::sweden,
    revenues::FeedInRevenue,
};

mod costs;
mod country;
mod currency;
mod defs;
mod fees;
mod links;
mod money;
mod power_tariffs;
mod registry;
mod revenues;

pub struct GridOperator {
    name: &'static str,
    price_date: NaiveDate,
    /// Costs are specified in this currency
    currency: Currency,
    /// The main fuse size range that this info covers
    main_fuses: MainFuseSizes,
    /// Fixed monthly fee
    monthly_fee: Cost,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Cost,
    transfer_fee: TransferFee,
    feed_in_revenue: FeedInRevenue,
    other_fees: OtherFees,
    power_tariff: Option<PowerTariff>,
    links: Links,
}

impl GridOperator {
    pub fn get(country: Country, name: &str) -> Option<&'static Self> {
        match country {
            Country::SE => sweden::SE_GRID_OPERATORS.iter().find(|o| o.name == name),
        }
    }

    pub fn power_tariff(&self) -> Option<&PowerTariff> {
        self.power_tariff.as_ref()
    }
}
