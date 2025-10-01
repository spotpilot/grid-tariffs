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
    costs::{Cost, CostPeriods, CostPeriodsSimple},
    country::{Country, CountryInfo},
    language::Language,
    links::*,
    money::Money,
    operator::{GridOperator, GridOperatorSimplified},
    power_tariffs::{PowerTariff, PowerTariffSimplified},
    price_list::PriceList,
    revenues::{FeedInRevenue, FeedInRevenueSimplified},
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
mod helpers;
mod language;
mod links;
mod money;
mod operator;
mod power_tariffs;
mod price_list;
pub mod registry;
mod revenues;
mod tax_reductions;
mod taxes;
mod transfer_fee;
