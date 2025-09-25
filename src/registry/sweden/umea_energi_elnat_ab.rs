use crate::registry::prelude::*;

pub static UMEA_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Umeå Energi Elnät AB")
    .vat_number("SE556086822501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder("https://www.umeaenergi.se/elnat/priser/priser-elnat")
            .plain_content_locator("main")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2538, 0).divide_by(12)),
            (20, Money::new(3799, 0).divide_by(12)),
            (25, Money::new(4739, 0).divide_by(12)),
            (35, Money::new(6398, 0).divide_by(12)),
            (50, Money::new(9374, 0).divide_by(12)),
            (63, Money::new(13125, 0).divide_by(12)),
            (80, Money::new(18595, 0).divide_by(12)),
            (100, Money::new(23355, 0).divide_by(12)),
            (125, Money::new(31015, 0).divide_by(12)),
            (160, Money::new(37989, 0).divide_by(12)),
            (200, Money::new(49509, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(21.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
