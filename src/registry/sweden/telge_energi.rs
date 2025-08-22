use crate::registry::prelude::*;

pub(super) const TELGE_ENERGI: GridOperator = GridOperator {
    name: "Telge Energi",
    price_date: date(2025, 1, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(415, 0)),
        (20, Money::new(451, 25)),
        (25, Money::new(496, 58)),
        (35, Money::new(587, 33)),
        (50, Money::new(723, 42)),
        (63, Money::new(841, 33)),
    ]),
    transfer_fee: TransferFee::fixed_subunit(7.0),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Simple(Cost::fixed_subunit(5.60)),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder()
            .fixed_cost(28, 0)
            .include_months(November, March)
            .include_hours(7, 20)
            .exclude_weekends_and_swedish_holidays()
            .finish()]),
    )),
    links: Links {
        fee_info: "https://www.telge.se/elnat/elnatskostnad/elnatspriser/",
        eltariff_api: None,
    },
};
