use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.nossebroenergi.se/sv/nattaxa";

pub static NOSSEBROORTENS_ENERGI_EKONOMISK_FORENING: GridOperator = GridOperator::builder()
    .name("Nossebroortens Energi Ekonomisk Förening")
    .vat_number("SE716410364501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".mainContainer")
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
