use crate::registry::prelude::*;

pub static VAXJO_ENERGI: GridOperator = GridOperator::builder()
    .name("Växjö Energi")
    .vat_number("SE556526851201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 1000))
    .links(
        Links::builder()
            .fee_info(
                "https://www.veab.se/privat/elnat/elnatsavgift/",
                "#main-content",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(248, 75)),
            (20, Money::new(400, 00)),
            (25, Money::new(486, 25)),
            (35, Money::new(660, 00)),
            (50, Money::new(920, 00)),
            (63, Money::new(1145, 00)),
            (80, Money::new(1607, 50)),
            (100, Money::new(1962, 50)),
            (125, Money::new(2386, 25)),
            (160, Money::new(2992, 50)),
            (200, Money::new(3685, 00)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::spot_price_variable(1.48, 0.0881, false))
        .transfer_fee(TransferFee::spot_price_variable(8.86, 0.1126, false))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new_first(&[CostPeriod::builder()
                .load(High)
                .cost(Cost::fuse_range(&[
                    (16, 63, Money::new(65, 48)),
                    (80, 1000, Money::new(91, 44)),
                ]))
                .hours(7, 20)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build()]),
        ))
        .build()])
    .build();
