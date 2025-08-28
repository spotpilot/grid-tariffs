use grid_tariffs::Country;

use crate::{
    LocatorMethod::*,
    locator::{ContentTarget, Locator, TargetContainer},
    pricing_info::{PricingInfo, PricingInfoRegistry},
};

pub(crate) static PRICING_INFO: PricingInfoRegistry = PricingInfoRegistry::new(&[
    PricingInfo {
        name: "EON",
        country: Country::SE,
        link: "https://www.eon.se/el/elnat/elnaetsabonnemang-priser",
        // NOTE: At the time of writing there are four tables with pricing info. Three are current and one is with old Stockholm prices.
        locator: Locator::new(
            CssSelector(r#"eon-ui-table-renderer"#),
            ContentTarget::Attribute("content"),
        ),
    },
    PricingInfo {
        name: "Ellevio",
        country: Country::SE,
        link: "https://www.ellevio.se/abonnemang/elnatspriser-privat/",
        locator: Locator::new(
            TextStartsWith {
                needle: "För bland annat villor, radhus, fritidshus och verksamhetslokaler med egen anslutning till elnätet – ej lägenheter.",
                target_container: TargetContainer::Parent,
            },
            ContentTarget::Text,
        ),
    },
    PricingInfo {
        name: "Vattenfall",
        country: Country::SE,
        link: "https://www.vattenfalleldistribution.se/abonnemang-och-avgifter/avtal-och-avgifter/elnatsavgift-och-avtalsvillkor/",
        locator: Locator::new(
            TextStartsWith {
                needle: "Säkringsabonnemang (16–63 A)",
                target_container: TargetContainer::Ancestor(1),
            },
            ContentTarget::Attribute("data-content"),
        ),
    },
    PricingInfo {
        name: "Göteborg Energi",
        country: Country::SE,
        link: "https://www.goteborgenergi.se/privat/elnat/elnatsavgiften",
        locator: Locator::new(CssSelector("#prisvilla + *"), ContentTarget::Text),
    },
    PricingInfo {
        name: "Kraftringen",
        country: Country::SE,
        link: "https://www.kraftringen.se/privat/elnat/elnatsavgifter/komplett-elnatsprislista/",
        locator: Locator::new(CssSelector(".main-page-content"), ContentTarget::Text),
    },
]);
