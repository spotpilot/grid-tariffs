use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://www.nackaenergi.se/privat/elnatspriser-1-juli--2025/priser-privat-1-juli-2025";

pub static NACKA_ENERGI: GridOperator = GridOperator::builder()
    .name("Nacka Energi")
    .vat_number("SE556017953201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 80)) // 80A - LSP also shares the same tariff (what is max for LSP?)
    .links(Links::builder().fee_info(FEE_LINK, "main").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 7, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(252, 75)),
            (20, Money::new(406, 92)),
            (25, Money::new(503, 75)),
            (35, Money::new(1033, 33)),
            (50, Money::new(1291, 67)),
            (63, Money::new(1808, 33)),
            (80, Money::new(2212, 67)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
            CostPeriod::builder()
                .load(High)
                .cost(Cost::fuse_range(&[
                    (16, 63, Money::new_subunit(33.7)),
                    (80, 80, Money::new_subunit(47.8)),
                ]))
                .months(November, March)
                .hours(6, 22)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build(),
            CostPeriod::builder()
                .load(Low)
                .cost(Cost::fuse_range(&[
                    (16, 63, Money::new_subunit(8.5)),
                    (80, 80, Money::new_subunit(12.)),
                ]))
                .build(),
        ])))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[CostPeriod::builder().load(Low).fixed_cost(51, 85).build()]),
        ))
        .build()])
    .build();
