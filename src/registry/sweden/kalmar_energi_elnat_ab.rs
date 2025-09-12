use crate::registry::prelude::*;

pub const KALMAR_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Kalmar Energi Elnät AB")
    .vat_number("SE556182754301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://kalmarenergi.se/foretag/el/elnat/elnatsavgifter/")
            .plain_content_locator("main")
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
