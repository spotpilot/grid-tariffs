use crate::registry::prelude::*;

pub static SKELLEFTEA_KRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Skellefteå Kraft Elnät AB")
    .vat_number("SE556244395101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.skekraft.se/privat/elnat/elnatspriser/")
            .plain_content_locator("section")
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
