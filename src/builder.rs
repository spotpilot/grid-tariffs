use crate::{
    Country, GridOperator,
    costs::Cost,
    currency::Currency,
    defs::MainFuseSizes,
    fees::TransferFee,
    helpers,
    links::Links,
    money::Money,
    power_tariffs::PowerTariff,
    price_list::{PriceList, PriceListBuilder},
    registry::sweden,
    revenues::FeedInRevenue,
};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct GridOperatorBuilder {
    name: Option<&'static str>,
    vat_number: Option<&'static str>,
    /// Costs are specified in this currency
    country: Option<Country>,
    /// The main fuse size range that this info covers
    main_fuses: Option<MainFuseSizes>,
    price_lists: Option<&'static [PriceList]>,
    links: Option<Links>,
}

impl GridOperatorBuilder {
    pub(crate) const fn new() -> Self {
        Self {
            name: None,
            vat_number: None,
            country: None,
            main_fuses: None,
            price_lists: None,
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

    pub(crate) const fn country(mut self, country: Country) -> Self {
        self.country = Some(country);
        self
    }

    pub(crate) const fn main_fuses(mut self, main_fuses: MainFuseSizes) -> Self {
        self.main_fuses = Some(main_fuses);
        self
    }

    pub(crate) const fn links(mut self, links: Links) -> Self {
        self.links = Some(links);
        self
    }

    pub(crate) const fn price_lists(mut self, price_lists: &'static [PriceList]) -> Self {
        self.price_lists = Some(price_lists);
        self
    }

    pub(crate) const fn build(self) -> GridOperator {
        GridOperator {
            name: self.name.expect("`name` required"),
            vat_number: self.vat_number.expect("`vat_number` required"),
            country: self.country.expect("`country` required"),
            main_fuses: self.main_fuses.expect("`main_fuses` required"),
            price_lists: self.price_lists.expect("`price_lists` expected"),
            links: self.links.expect("`links` required"),
        }
    }
}
