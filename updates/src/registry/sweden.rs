use grid_tariffs::registry::sweden;

use crate::{
    LocatorMethod::*,
    locator::{ContentTarget, Locator, TargetContainer},
    pricing_info::{PricingInfo, PricingInfoRegistry},
};

pub(crate) static PRICING_INFO: PricingInfoRegistry = PricingInfoRegistry::new(&[
    PricingInfo::builder(&sweden::eon::SYD)
        .name_override("EON")
        .locator(Locator::new(
            CssSelector(r#"eon-ui-table-renderer"#),
            ContentTarget::Attribute("content"),
        ))
        .build(),
    PricingInfo::builder(&sweden::ellevio::ELLEVIO)
        .locator(Locator::new(
            TextStartsWith {
                needle: "För bland annat villor, radhus, fritidshus och verksamhetslokaler med egen anslutning till elnätet – ej lägenheter.",
                target_container: TargetContainer::Parent,
            },
            ContentTarget::Text,
        ))
        .build(),
    PricingInfo::builder(&sweden::vattenfall::VATTENFALL_E4)
        .name_override("Vattenfall")
        .locator(Locator::new(
            TextStartsWith {
                needle: "Säkringsabonnemang (16–63 A)",
                target_container: TargetContainer::Ancestor(1),
            },
            ContentTarget::Attribute("data-content"),
        ))
        .build(),
    PricingInfo::builder(&sweden::goteborg_energi::GÖTEBORG_ENERGI)
        .locator(Locator::new(CssSelector("#prisvilla + *"), ContentTarget::Text))
        .build(),
    PricingInfo::builder(&sweden::kraftringen::KRAFTRINGEN)
        .locator(Locator::new(
            CssSelector(".main-page-content"),
            ContentTarget::Text,
        ))
        .build()
]);
