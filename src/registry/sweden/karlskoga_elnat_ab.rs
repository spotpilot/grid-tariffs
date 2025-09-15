use crate::registry::prelude::*;

pub static KARLSKOGA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Karlskoga Elnät AB")
    .vat_number("SE556507429001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(
            "https://www.karlskogaenergi.se/Vara-tjanster/elnat/priser-och-avtalsvillkor/",
        )
        .plain_content_locator("#mainContent")
        .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
