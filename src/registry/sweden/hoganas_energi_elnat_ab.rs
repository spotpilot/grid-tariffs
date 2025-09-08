use crate::registry::prelude::*;

pub const HOGANAS_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Höganäs Energi Elnät AB")
    .vat_number("SE556440240101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.hoganasenergi.se/elnat/prislistor")
            .plain_content_locator(".pagecontent")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
