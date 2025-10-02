use chrono::NaiveDate;
use serde::Serialize;

use crate::{
    FeedInRevenueSimplified, Language, Money, PowerTariff, PowerTariffSimplified,
    TransferFeeSimplified, costs::Cost, feed_in_revenue::FeedInRevenue, helpers,
    transfer_fee::TransferFee,
};

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PriceList {
    variant: Option<&'static str>,
    from_date: NaiveDate,
    /// Fixed monthly fee
    monthly_fee: Cost,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Cost,
    transfer_fee: TransferFee,
    feed_in_revenue: FeedInRevenue,
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

    pub const fn power_tariff(&self) -> &PowerTariff {
        &self.power_tariff
    }

    pub fn simplified(
        &self,
        fuse_size: u16,
        yearly_consumption: u32,
        language: Language,
    ) -> PriceListSimplified {
        PriceListSimplified::new(self, fuse_size, yearly_consumption, language)
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
            power_tariff: self.power_tariff.expect("`grid_tariff` required"),
        }
    }

    pub(crate) const fn variant(mut self, name: &'static str) -> Self {
        self.variant = Some(name);
        self
    }

    #[allow(clippy::wrong_self_convention)]
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

    pub(crate) const fn power_tariff(mut self, power_tariff: PowerTariff) -> Self {
        self.power_tariff = Some(power_tariff);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PriceListSimplified {
    variant: Option<&'static str>,
    fuse_size: u16,
    /// If the energy consumption of the household affects the costs and/or fees for this fuse size
    yearly_consumption_based: bool,
    from_date: NaiveDate,
    /// Fixed monthly fee
    monthly_fee: Option<Money>,
    /// Fixed monthly fee for allowing energy production
    monthly_production_fee: Option<Money>,
    transfer_fee: TransferFeeSimplified,
    feed_in_revenue: FeedInRevenueSimplified,
    power_tariff: PowerTariffSimplified,
}

impl PriceListSimplified {
    fn new(pl: &PriceList, fuse_size: u16, yearly_consumption: u32, language: Language) -> Self {
        Self {
            variant: pl.variant,
            fuse_size,
            yearly_consumption_based: pl.monthly_fee.is_yearly_consumption_based(fuse_size)
                || pl
                    .monthly_production_fee
                    .is_yearly_consumption_based(fuse_size)
                || pl.transfer_fee.is_yearly_consumption_based(fuse_size),
            from_date: pl.from_date,
            monthly_fee: pl.monthly_fee.cost_for(fuse_size, yearly_consumption),
            monthly_production_fee: pl
                .monthly_production_fee
                .cost_for(fuse_size, yearly_consumption),
            transfer_fee: pl
                .transfer_fee
                .simplified(fuse_size, yearly_consumption, language),
            feed_in_revenue: pl
                .feed_in_revenue
                .simplified(fuse_size, yearly_consumption, language),
            power_tariff: pl
                .power_tariff
                .simplified(fuse_size, yearly_consumption, language),
        }
    }
}
