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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brittedal_operator_exists() {
        let operator = GridOperator::get(Country::SE, "Brittedal");
        assert!(operator.is_some(), "Brittedal operator should be found");
        
        let operator = operator.unwrap();
        assert_eq!(operator.name, "Brittedal");
    }

    #[test]
    fn test_brittedal_operator_properties() {
        let operator = GridOperator::get(Country::SE, "Brittedal").unwrap();
        
        // Test that we can access basic properties (name is tested via the get method working)
        // Most other properties are private, but we can test that the operator exists and has the right name
        assert_eq!(operator.name, "Brittedal");
    }

    #[test]
    fn test_brittedal_integration_demo() {
        // Demonstrate that Brittedal operator works alongside other operators
        let brittedal = GridOperator::get(Country::SE, "Brittedal");
        assert!(brittedal.is_some(), "Brittedal should be found");
        
        let btea = GridOperator::get(Country::SE, "BTEA");
        assert!(btea.is_some(), "BTEA should still be found");
        
        let bjarke = GridOperator::get(Country::SE, "Bjärke Energi");
        assert!(bjarke.is_some(), "Bjärke Energi should still be found");
        
        // Verify they have different names (Brittedal is distinct from BTEA as mentioned in comments)
        assert_ne!(brittedal.unwrap().name, btea.unwrap().name);
    }
}
