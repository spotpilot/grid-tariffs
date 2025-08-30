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
            ContentTarget::TextWithLinks,
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
        .locator(Locator::new(CssSelector("#prisvilla + *"), ContentTarget::TextWithLinks))
        .build(),
    PricingInfo::builder(&sweden::kraftringen::KRAFTRINGEN)
        .locator(Locator::new(
            CssSelector(".main-page-content"),
            ContentTarget::TextWithLinks,
        ))
        .build(),
    PricingInfo::builder(&sweden::bjarke_energi::BJÄRKE_ENERGI).locator_simple("h2 ~ table").build(),
    PricingInfo::builder(&sweden::btea::BTEA).locator_simple("table").build(),
    PricingInfo::builder(&sweden::eskilstuna_energi_miljo::ESKILSTUNA_ENERGI_MILJÖ).locator_simple("article").build(),
    PricingInfo::builder(&sweden::falbygdens_energi::FALBYGDENS_ENERGI).locator_simple(".pagecontent").build(),
    PricingInfo::builder(&sweden::gislaved_energi::GISLAVED_ENERGI).locator_simple("#page-content").build(),
    PricingInfo::builder(&sweden::jbf::JBF).locator_simple("#main").build(),
    PricingInfo::builder(&sweden::jonkoping_energi::JÖNKÖPING_ENERGI).locator_simple("section").build(),
    PricingInfo::builder(&sweden::karlstads_energi::KARLSTADS_ENERGI).locator_simple("table").build(),
    PricingInfo::builder(&sweden::kristinehamns_energi::KRISTINEHAMNS_ENERGI).locator_simple("section").build(),
    PricingInfo::builder(&sweden::linde_energi::LINDE_ENERGI).locator_simple("#Innehall + div").build(),
    PricingInfo::builder(&sweden::malarenergi::MÄLARENERGI).locator_simple(".standard-article").build(),
    PricingInfo::builder(&sweden::malung_salens_elverk::MALUNG_SÄLENS_ELVERK).locator_simple(".content:nth-child(1)").build(),
    PricingInfo::builder(&sweden::nacka_energi::NACKA_ENERGI).locator_simple("main").build(),
    PricingInfo::builder(&sweden::partille_energi::PARTILLE_ENERGI).locator_simple("#elnatsavtal").build(),
    PricingInfo::builder(&sweden::skovde_energi::SKÖVDE_ENERGI).locator_simple("main").build(),
    PricingInfo::builder(&sweden::sollentuna_energi_miljo_ab::SOLLENTUNA_ENERGI_MILJÖ_AB).locator_simple(".main-content-area").build(),
    PricingInfo::builder(&sweden::telge_energi::TELGE_ENERGI).locator_simple("main").build(),
    PricingInfo::builder(&sweden::vaxjo_energi::VÄXJÖ_ENERGI).locator_simple("#main-content").build(),
    PricingInfo::builder(&sweden::tekniska_verken::KATRINEHOLM_ALTERNATIV).name_override("Tekniska verken").locator_simple("main").build(),
]);
