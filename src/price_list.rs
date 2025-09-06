use chrono::NaiveDate;
use serde::Serialize;

use crate::{
    costs::Cost,
    fees::{OtherFees, TransferFee},
    helpers,
    power_tariffs::PowerTariff,
    revenues::FeedInRevenue,
};

#[derive(Debug, Clone, Serialize)]
pub struct PriceList {
    variant: Option<&'static str>,
    from_date: NaiveDate,
    /// Fixed monthly fee
    monthly_fee: Cost,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Cost,
    transfer_fee: TransferFee,
    feed_in_revenue: FeedInRevenue,
    other_fees: OtherFees,
    power_tariff: PowerTariff,
}

impl PriceList {
    pub(crate) const fn builder() -> PriceListBuilder {
        PriceListBuilder::new()
    }

    pub const fn variant(&self) -> Option<&'static str> {
        self.variant
    }

    pub const fn from_date(&self) -> NaiveDate {
        self.from_date
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
}

#[derive(Debug, Clone)]
pub(crate) struct PriceListBuilder {
    variant: Option<&'static str>,
    from_date: Option<NaiveDate>,
    /// Fixed monthly fee
    monthly_fee: Option<Cost>,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Option<Cost>,
    transfer_fee: Option<TransferFee>,
    feed_in_revenue: Option<FeedInRevenue>,
    other_fees: Option<OtherFees>,
    power_tariff: Option<PowerTariff>,
}

impl PriceListBuilder {
    pub(crate) const fn new() -> Self {
        Self {
            variant: None,
            from_date: None,
            monthly_fee: None,
            monthly_production_fee: None,
            transfer_fee: None,
            feed_in_revenue: None,
            other_fees: None,
            power_tariff: None,
        }
    }

    pub(crate) const fn build(self) -> PriceList {
        PriceList {
            variant: self.variant,
            from_date: self.from_date.expect("`from_date` required"),
            monthly_fee: self.monthly_fee.expect("`monthly_fee` required"),
            monthly_production_fee: self
                .monthly_production_fee
                .expect("`monthly_production_fee` required"),
            transfer_fee: self.transfer_fee.expect("`transfer_fee` required"),
            feed_in_revenue: self.feed_in_revenue.expect("`feed_in_revenue` required"),
            other_fees: self.other_fees.expect("`other_fees` required"),
            power_tariff: self.power_tariff.expect("`grid_tariff` required"),
        }
    }

    pub(crate) const fn variant(mut self, name: &'static str) -> Self {
        self.variant = Some(name);
        self
    }

    pub(crate) const fn from_date(mut self, year: i32, month: u32, day: u32) -> Self {
        self.from_date = Some(helpers::date(year, month, day));
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
}
