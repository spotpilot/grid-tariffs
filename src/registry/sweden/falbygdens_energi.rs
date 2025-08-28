use crate::registry::prelude::*;

pub const FALBYGDENS_ENERGI: GridOperator = GridOperator {
    name: "Falbygdens Energi",
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fixed(343, 42),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(37.2),
    other_fees: OtherFees::Unverified,
    links: Links::new("https://falbygdensenergi.se/privat/vart-elnat/elnatsavgifter-samt-villkor"),
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDaysDifferentiated { base: 1, peak: 1 },
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(39, 80)
                .fallthrough(true)
                .build(),
            CostPeriod::builder()
                .fixed_cost(57, 36)
                .include_months(November, March)
                .include_hours(7, 19)
                .exclude_weekends_and_swedish_holidays()
                .build(),
        ]),
    )),
};
