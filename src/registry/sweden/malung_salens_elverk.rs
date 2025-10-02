use crate::registry::prelude::*;
pub static MALUNG_SALENS_ELVERK: GridOperator = GridOperator::builder()
    .name("Malung-Sälens Elverk")
    .vat_number("SE556527481701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://malungselnat.se/elnat/nattariffer/",
                ".content:nth-child(1)",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 4, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(283, 75)),
            (20, Money::new(440, 42)),
            (25, Money::new(617, 8)),
            (35, Money::new(873, 33)),
            (50, Money::new(1128, 75)),
            (63, Money::new(1384, 58)),
        ]))
        // NOTE: Couldn't find any transfer fee listed on their website
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unlisted)
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(5),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(118, 75)
                    .months(November, March)
                    .hours(7, 19)
                    .build(),
                CostPeriod::builder().load(Low).fixed_cost(35, 0).build(),
            ]),
        ))
        .build()])
    .build();
