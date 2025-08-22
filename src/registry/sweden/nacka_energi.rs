use crate::registry::prelude::*;

pub(super) const NACKA_ENERGI: GridOperator = GridOperator {
    name: "Nacka Energi",
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 80), // 80A - LSP also shares the same tariff (don't know what LSP is)
    price_date: date(2025, 7, 1),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(252, 75)),
        (20, Money::new(406, 92)),
        (25, Money::new(503, 75)),
        (35, Money::new(1033, 33)),
        (50, Money::new(1291, 67)),
        (63, Money::new(1808, 33)),
        (80, Money::new(2212, 67)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::new_periods(CostPeriods::new(&[
        CostPeriod::builder()
            .cost(Cost::fuse_range(&[
                (16, 63, Money::new_subunit(33.7)),
                (80, 80, Money::new_subunit(47.8)),
            ]))
            .include_months(November, March)
            .include_hours(6, 22)
            .exclude_weekends_and_swedish_holidays()
            .finish(),
        CostPeriod::builder()
            .cost(Cost::fuse_range(&[
                (16, 63, Money::new_subunit(8.5)),
                (80, 80, Money::new_subunit(12.)),
            ]))
            .finish(),
    ])),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://www.nackaenergi.se/privat/elnatspriser-1-juli--2025/priser-privat-1-juli-2025",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageHours(3),
        CostPeriods::new(&[CostPeriod::builder().fixed_cost(51, 85).finish()]),
    )),
};
