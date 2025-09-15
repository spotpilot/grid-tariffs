use crate::registry::prelude::*;

pub static DALA_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Dala Energi Elnät AB")
    .vat_number("SE556166775801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://dalaenergi.se/el/avgift/")
            .plain_content_locator("*:has(> #priser)")
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
