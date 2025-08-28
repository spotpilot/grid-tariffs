use crate::registry::prelude::*;
pub const SOLLENTUNA_ENERGI_MILJÖ_AB: GridOperator = GridOperator {
    name: "Sollentuna Energi & Miljö AB",
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 200),
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(137, 50)),
        (25, Money::new(137, 50)),
        (35, Money::new(245, 0)),
        (50, Money::new(345, 0)),
        (63, Money::new(420, 0)),
        (80, Money::new(536, 67)),
        (100, Money::new(665, 0)),
        (125, Money::new(816, 67)),
        (160, Money::new(1050, 0)),
        (200, Money::new(1330, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Unlisted,
    other_fees: OtherFees::List(&[("Energiavgift", Cost::fixed_subunit(5.))]),
    links: Links::new("https://www.seom.se/el/elnat/priser-och-villkor/"),
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageHours(3),
        CostPeriods::new(&[
            CostPeriod::builder()
                .cost(Cost::None)
                .include(PeriodType::Weekends)
                .include(PeriodType::SwedishHolidays)
                .build(),
            CostPeriod::builder()
                .cost(Cost::fixed(135, 0))
                .include_months(November, March)
                .include_hours(7, 19)
                .exclude_weekends_and_swedish_holidays()
                .build(),
            CostPeriod::builder()
                .cost(Cost::fixed(67, 50))
                .include_months(April, October)
                .include_hours(7, 19)
                .exclude_weekends_and_swedish_holidays()
                .build(),
        ]),
    )),
};
