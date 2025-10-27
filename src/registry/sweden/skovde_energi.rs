use crate::registry::prelude::*;

pub static SKOVDE_ENERGI: GridOperator = GridOperator::builder()
    .name("Skövde Energi")
    .vat_number("SE556959114101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://skovdeenergi.se/elnat/taxor-avgifter/elnatstaxa-priser-med-moms/",
                "main",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(125, 0)),
            (20, Money::new(198, 42)),
            (25, Money::new(234, 92)),
            (35, Money::new(271, 33)),
            (50, Money::new(313, 0)),
            (63, Money::new(385, 92)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::new_periods(CostPeriods::new_first(&[
            CostPeriod::builder()
                .load(High)
                .cost(Cost::fuse_range(&[
                    (16, 25, Money::new_subunit(26.25)),
                    (35, 63, Money::new_subunit(20.)),
                ]))
                .months(November, March)
                .hours(6, 22)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build(),
            CostPeriod::builder()
                .load(Low)
                .fixed_cost_subunit(5.5)
                .build(),
        ])))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(1),
            CostPeriods::new_first(&[CostPeriod::builder()
                .load(High)
                .cost(Cost::fuse_range(&[
                    (16, 25, Money::new(77, 5)),
                    (35, 63, Money::new(128, 8)),
                ]))
                .months(November, March)
                .hours(6, 22)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build()]),
        ))
        .build()])
    .build();
