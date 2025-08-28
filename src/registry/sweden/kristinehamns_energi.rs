use crate::registry::prelude::*;

pub const KRISTINEHAMNS_ENERGI: GridOperator = GridOperator {
    name: "Kristinehamns Energi",
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fixed(277, 50),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(12.75),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://kristinehamnsenergi.se/elnat/elnatsavgiften/din-elnatsavgift/",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(43, 75)
                .fallthrough(true)
                .build(),
            CostPeriod::builder()
                .fixed_cost(91, 25)
                .include_months(November, March)
                .include_hours(7, 17)
                .exclude_weekends_and_swedish_holidays()
                .build(),
        ]),
    )),
};
