use crate::{
    Country, GridOperator,
    costs::Cost,
    currency::Currency,
    defs::MainFuseSizes,
    fees::{OtherFees, TransferFee},
    helpers,
    links::Links,
    money::Money,
    power_tariffs::PowerTariff,
    registry::sweden,
    revenues::FeedInRevenue,
};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct GridOperatorBuilder {
    name: Option<&'static str>,
    vat_number: Option<&'static str>,
    price_date: Option<NaiveDate>,
    /// Costs are specified in this currency
    country: Option<Country>,
    /// The main fuse size range that this info covers
    main_fuses: Option<MainFuseSizes>,
    /// Fixed monthly fee
    monthly_fee: Option<Cost>,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Option<Cost>,
    transfer_fee: Option<TransferFee>,
    feed_in_revenue: Option<FeedInRevenue>,
    other_fees: Option<OtherFees>,
    power_tariff: Option<PowerTariff>,
    links: Option<Links>,
}

impl GridOperatorBuilder {
    pub(crate) const fn new() -> Self {
        Self {
            name: None,
            vat_number: None,
            price_date: None,
            country: None,
            main_fuses: None,
            monthly_fee: None,
            monthly_production_fee: None,
            transfer_fee: None,
            feed_in_revenue: None,
            other_fees: None,
            power_tariff: None,
            links: None,
        }
    }

    pub(crate) const fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub(crate) const fn vat_number(mut self, vat_number: &'static str) -> Self {
        self.vat_number = Some(vat_number);
        self
    }

    pub(crate) const fn price_date(mut self, year: i32, month: u32, day: u32) -> Self {
        self.price_date = Some(helpers::date(year, month, day));
        self
    }

    pub(crate) const fn country(mut self, country: Country) -> Self {
        self.country = Some(country);
        self
    }

    pub(crate) const fn main_fuses(mut self, main_fuses: MainFuseSizes) -> Self {
        self.main_fuses = Some(main_fuses);
        self
    }

    pub(crate) const fn monthly_fee(mut self, monthly_fee: Cost) -> Self {
        self.monthly_fee = Some(monthly_fee);
        self
    }

    pub(crate) const fn monthly_production_fee(mut self, monthly_production_fee: Cost) -> Self {
        self.monthly_production_fee = Some(monthly_production_fee);
        self
    }

    pub(crate) const fn transfer_fee(mut self, transfer_fee: TransferFee) -> Self {
        self.transfer_fee = Some(transfer_fee);
        self
    }

    pub(crate) const fn feed_in_revenue(mut self, feed_in_revenue: FeedInRevenue) -> Self {
        self.feed_in_revenue = Some(feed_in_revenue);
        self
    }

    pub(crate) const fn other_fees(mut self, other_fees: OtherFees) -> Self {
        self.other_fees = Some(other_fees);
        self
    }

    pub(crate) const fn power_tariff(mut self, power_tariff: PowerTariff) -> Self {
        self.power_tariff = Some(power_tariff);
        self
    }

    pub(crate) const fn links(mut self, links: Links) -> Self {
        self.links = Some(links);
        self
    }

    pub(crate) const fn build(self) -> GridOperator {
        GridOperator {
            name: self.name.expect("`name` required"),
            vat_number: self.vat_number.expect("`vat_number` required"),
            price_date: self.price_date.expect("`price_date` required"),
            country: self.country.expect("`country` required"),
            main_fuses: self.main_fuses.expect("`main_fuses` required"),
            monthly_fee: self.monthly_fee.expect("`monthly_fee` required"),
            monthly_production_fee: self
                .monthly_production_fee
                .expect("`monthly_production_fee` required"),
            transfer_fee: self.transfer_fee.expect("`transfer_fee` required"),
            feed_in_revenue: self.feed_in_revenue.expect("`feed_in_revenue` required"),
            other_fees: self.other_fees.expect("`other_fees` required"),
            power_tariff: self.power_tariff,
            links: self.links.expect("`links` required"),
        }
    }
}
