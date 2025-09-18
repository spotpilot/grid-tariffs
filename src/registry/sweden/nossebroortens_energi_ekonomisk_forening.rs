use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.nossebroenergi.se/sv/nattaxa";

pub static NOSSEBROORTENS_ENERGI_EKONOMISK_FORENING: GridOperator = GridOperator::builder()
    .name("Nossebroortens Energi Ekonomisk Förening")
    .vat_number("SE716410364501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 25))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".mainContainer")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4888, 0).divide_by(12)),
            (20, Money::new(5850, 0).divide_by(12)),
            (25, Money::new(6900, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(33.80))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
