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

pub use crate::{
    costs::*,
    country::{Country, CountryInfo},
    feed_in_revenue::{FeedInRevenue, FeedInRevenueSimplified},
    language::Language,
    links::*,
    money::Money,
    operator::{GridOperator, GridOperatorSimplified},
    peaks::*,
    power_tariffs::{PowerTariff, PowerTariffSimplified, TariffCalculationMethod},
    price_list::PriceList,
    tax_reductions::*,
    taxes::*,
    transfer_fee::{TransferFee, TransferFeeSimplified},
};
use crate::{
    currency::Currency, defs::MainFuseSizes, price_list::PriceListSimplified, registry::sweden,
};
pub(crate) use operator::GridOperatorBuilder;

mod costs;
mod country;
mod currency;
mod defs;
mod feed_in_revenue;
mod helpers;
mod language;
mod links;
mod money;
mod operator;
mod peaks;
mod power_tariffs;
mod price_list;
pub mod registry;
mod tax_reductions;
mod taxes;
mod transfer_fee;
