use crate::registry::prelude::*;

pub(super) const MÄLARENERGI: GridOperator = GridOperator {
    name: "Mälarenergi",
    price_date: date(2025, 9, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(307, 50)),
        (20, Money::new(335, 0)),
        (25, Money::new(362, 50)),
        (35, Money::new(437, 50)),
        (50, Money::new(540, 0)),
        (63, Money::new(625, 0)),
    ]),
    transfer_fee: TransferFee::fixed_subunit(36.25),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::new_periods(CostPeriods::new(&[
        CostPeriod::builder()
            .fixed_cost_subunit(9.11)
            .include_months(November, March)
            .include_hours(6, 22)
            .exclude_weekends()
            .finish(),
        CostPeriod::builder().fixed_cost_subunit(4.91).finish(),
    ])),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder()
            .fixed_cost(18, 75)
            .include_hours(7, 18)
            .exclude_weekends_and_swedish_holidays()
            .finish()]),
    )),
    links: Links {
        fee_info: "https://www.malarenergi.se/el/elnat/effektsmart/prismodellen/",
        eltariff_api: None,
    },
};
