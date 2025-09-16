use chrono::Utc;
use indexmap::IndexMap;
use serde::Serialize;

use crate::{
    Country, Links, MainFuseSizes, PriceList, currency::Currency, price_list::PriceListSimplified,
    registry::sweden,
};

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
            Country::SE => sweden::GRID_OPERATORS
                .iter()
                .find(|o| o.name == name)
                .copied(),
        }
    }

    pub fn all() -> Vec<&'static Self> {
        sweden::GRID_OPERATORS.iter().copied().collect()
    }

    pub fn all_for_country(country: Country) -> &'static [&'static Self] {
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

#[derive(Debug, Clone)]
pub(crate) struct GridOperatorBuilder {
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
