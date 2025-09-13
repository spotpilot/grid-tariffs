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
use std::collections::HashMap;

use chrono::{NaiveDate, Utc};
use indexmap::IndexMap;
use serde::Serialize;

use crate::{
    builder::GridOperatorBuilder, currency::Currency, defs::MainFuseSizes,
    price_list::PriceListSimplified, registry::sweden,
};
pub use crate::{
    costs::Cost,
    country::{Country, CountryInfo},
    fees::{TransferFee, TransferFeeSimplified},
    links::*,
    money::Money,
    power_tariffs::PowerTariff,
    price_list::PriceList,
    revenues::{FeedInRevenue, FeedInRevenueSimplified},
    tax_reductions::*,
    taxes::*,
};

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
mod price_list;
pub mod registry;
mod revenues;
mod tax_reductions;
mod taxes;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GridOperator {
    name: &'static str,
    vat_number: &'static str,
    /// Costs are specified in this currency
    country: Country,
    /// The main fuse size range that this info covers
    main_fuses: MainFuseSizes,
    price_lists: &'static [PriceList],
    links: Links,
}

impl GridOperator {
    pub const fn name(&self) -> &str {
        &self.name
    }

    pub const fn vat_number(&self) -> &str {
        &self.vat_number
    }

    pub const fn country(&self) -> Country {
        self.country
    }

    pub const fn links(&self) -> &Links {
        &self.links
    }

    pub fn active_price_lists(&self) -> Vec<&'static PriceList> {
        let now = Utc::now().date_naive();
        let mut map: IndexMap<Option<&str>, &PriceList> = IndexMap::new();
        for pl in self.price_lists {
            if now >= pl.from_date() {
                if let Some(current_max_date) = map.get(&pl.variant()).map(|pl| pl.from_date()) {
                    if pl.from_date() > current_max_date {
                        map.insert(pl.variant(), pl);
                    }
                } else {
                    map.insert(pl.variant(), pl);
                }
            }
        }
        map.into_values().collect()
    }

    pub fn active_price_list(&self, variant: Option<&str>) -> Option<&'static PriceList> {
        self.active_price_lists()
            .iter()
            .filter(|pl| pl.variant() == variant)
            .last()
            .copied()
    }

    pub fn price_lists(&self) -> &'static [PriceList] {
        self.price_lists
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

    pub fn all() -> Vec<&'static Self> {
        sweden::GRID_OPERATORS.iter().collect()
    }

    pub fn all_for_country(country: Country) -> &'static [Self] {
        match country {
            Country::SE => sweden::GRID_OPERATORS,
        }
    }

    pub(crate) const fn builder() -> GridOperatorBuilder {
        GridOperatorBuilder::new()
    }

    pub fn simplified(&self, fuse_size: u16, yearly_consumption: u32) -> GridOperatorSimplified {
        GridOperatorSimplified::new(self, fuse_size, yearly_consumption)
    }
}

/// Grid operator with only current prices
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GridOperatorSimplified {
    name: &'static str,
    vat_number: &'static str,
    /// Costs are specified in this currency
    country: Country,
    price_lists: Vec<PriceListSimplified>,
}

impl GridOperatorSimplified {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn vat_number(&self) -> &'static str {
        self.vat_number
    }

    pub fn country(&self) -> Country {
        self.country
    }

    pub fn price_lists(&self) -> &[PriceListSimplified] {
        &self.price_lists
    }
}

impl GridOperatorSimplified {
    fn new(op: &GridOperator, fuse_size: u16, yearly_consumption: u32) -> Self {
        Self {
            name: op.name,
            vat_number: op.vat_number,
            country: op.country(),
            price_lists: op
                .active_price_lists()
                .into_iter()
                .map(|pl| pl.simplified(fuse_size, yearly_consumption))
                .collect(),
        }
    }
}
