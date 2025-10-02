use crate::registry::prelude::*;

const FEE_LINK: &str = "https://tranasenergi.se/privat/elnat/priser-och-villkor/";

pub static TRANAS_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Tranås Energi Elnät AB")
    .vat_number("SE556952020701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 35))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4970, 0).divide_by(12)),
            (20, Money::new(5872, 50).divide_by(12)),
            (25, Money::new(6996, 25).divide_by(12)),
            (35, Money::new(9363, 75).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(33.6))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
