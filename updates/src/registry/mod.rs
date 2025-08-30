use grid_tariffs::Country;

use crate::pricing_info::PricingInfo;

pub(crate) mod sweden;

pub(crate) fn get(country: Country, name: &str) -> Option<&'static PricingInfo> {
    match country {
        Country::SE => sweden::PRICING_INFO.get(name),
    }
}

pub(crate) fn starts_with(country: Country, text: &str) -> Vec<&'static PricingInfo> {
    match country {
        Country::SE => sweden::PRICING_INFO.starts_with(text),
    }
}

pub(crate) fn all_pricing_info() -> Vec<&'static PricingInfo> {
    sweden::PRICING_INFO.iter().collect()
}
