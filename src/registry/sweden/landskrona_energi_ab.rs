use crate::registry::prelude::*;

pub const LANDSKRONA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Landskrona Energi AB")
    .vat_number("SE556803921701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://landskronaenergi.se/privat/el/elnatsavgifter/")
            .plain_content_locator(".elementor-price-list")
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
