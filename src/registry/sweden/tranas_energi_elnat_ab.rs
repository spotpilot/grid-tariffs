use crate::registry::prelude::*;

const FEE_LINK: &str = "https://tranasenergi.se/foretag/elnat/priser-och-villkor/";

pub static TRANAS_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Tranås Energi Elnät AB")
    .vat_number("SE556952020701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3976, 0).divide_by(12)),
            (20, Money::new(4698, 0).divide_by(12)),
            (25, Money::new(5597, 0).divide_by(12)),
            (35, Money::new(7491, 0).divide_by(12)),
            (50, Money::new(10305, 0).divide_by(12)),
            (63, Money::new(12941, 0).divide_by(12)),
            (80, Money::new(16225, 0).divide_by(12)),
            (100, Money::new(20316, 0).divide_by(12)),
            (125, Money::new(25169, 0).divide_by(12)),
            (160, Money::new(32262, 0).divide_by(12)),
            (200, Money::new(39584, 0).divide_by(12)),
            (250, Money::new(47900, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(26.9))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
