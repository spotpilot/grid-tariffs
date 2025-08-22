use crate::registry::prelude::*;

pub(super) const JÖNKÖPING_ENERGI: GridOperator = GridOperator {
    name: "Jönköping Energi",
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    price_date: date(2025, 9, 1),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(296, 0)),
        (20, Money::new(351, 0)),
        (25, Money::new(420, 0)),
        (35, Money::new(557, 0)),
        (50, Money::new(762, 0)),
        (63, Money::new(941, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(6.08),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://jonkopingenergi.se/privat/elnat/elnat/priser",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDaysDifferentiated { base: 2, peak: 2 },
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(27, 70)
                .fallthrough(true)
                .finish(),
            CostPeriod::builder()
                .cost(Cost::fixed(65, 66))
                .include_months(November, March)
                .include_hours(7, 20)
                .exclude_weekends_and_swedish_holidays()
                .finish(),
        ]),
    )),
};
