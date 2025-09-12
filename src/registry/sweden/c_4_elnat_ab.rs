use crate::registry::prelude::*;

pub const C_4_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("C4 Elnät AB")
    .vat_number("SE556496004401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.c4energi.se/privat/elnat/priser-villkor-avtal/")
            .plain_content_locator(".content")
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
