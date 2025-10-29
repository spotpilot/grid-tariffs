use crate::registry::prelude::*;

pub static NATKRAFT_BORAS_INFRA_AB: GridOperator = GridOperator::builder()
    .name("Nätkraft Borås Infra AB")
    .vat_number("SE556527558201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info("https://natkraftboras.se/elnat/elnatsavtal/priser/", "main")
            .feed_in_revenue_info_default("https://natkraftboras.se/elnat/elnatsavtal/priser/")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 9, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3700, 0).divide_by(12)),
            (20, Money::new(6475, 0).divide_by(12)),
            (25, Money::new(570, 0)),
            (35, Money::new(775, 0)),
            (50, Money::new(1100, 0)),
            (63, Money::new(1370, 0)),
            (80, Money::new(1750, 0)),
            (100, Money::new(2165, 0)),
            (125, Money::new(2695, 0)),
            (160, Money::new(3410, 0)),
            (200, Money::new(4235, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(5.5))
        .transfer_fee(TransferFee::Simple(Cost::fuse_range(&[
            (16, 20, Money::new_subunit(12.56)),
            (25, 200, Money::new_subunit(6.0)),
        ])))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            // NOTE: This doesn't apply for fuse sizes 16-20
            // TODO: https://github.com/spotpilot/grid-tariffs/issues/177
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost(15, 30)
                    .months(April, October)
                    .build(),
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(35, 20)
                    .months(November, March)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .hours(7, 18)
                    .build(),
            ]),
        ))
        .build()])
    .build();
