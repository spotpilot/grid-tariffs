use crate::registry::prelude::*;

pub static C_4_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("C4 Elnät AB")
    .vat_number("SE556496004401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info(
                "https://www.c4energi.se/privat/elnat/priser-villkor-avtal/",
                ".content",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 7, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(6273, 0).divide_by(12)),
            (20, Money::new(8604, 0).divide_by(12)),
            (25, Money::new(10691, 0).divide_by(12)),
            (35, Money::new(15540, 0).divide_by(12)),
            (50, Money::new(22893, 0).divide_by(12)),
            (63, Money::new(30135, 0).divide_by(12)),
            (80, Money::new(42720, 0).divide_by(12)),
            (100, Money::new(59750, 0).divide_by(12)),
            (125, Money::new(84745, 0).divide_by(12)),
            (160, Money::new(114402, 0).divide_by(12)),
            (200, Money::new(149144, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::spot_price_variable(5.0, 6.13, false))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
