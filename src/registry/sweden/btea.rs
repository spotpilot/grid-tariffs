use crate::registry::prelude::*;

pub const BTEA: GridOperator = GridOperator {
    name: "BTEA",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(628, 25)),  // 7539 kr/år ÷ 12 = 628.25 kr/månad
        (20, Money::new(761, 25)),  // 9135 kr/år ÷ 12 = 761.25 kr/månad
        (25, Money::new(883, 17)),  // 10598 kr/år ÷ 12 = 883.17 kr/månad
        (35, Money::new(1492, 50)), // 17910 kr/år ÷ 12 = 1492.50 kr/månad
        (50, Money::new(2093, 67)), // 25124 kr/år ÷ 12 = 2093.67 kr/månad
        (63, Money::new(2593, 67)), // 31124 kr/år ÷ 12 = 2593.67 kr/månad
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Simple(Cost::fixed_subunit(3.75)), // 3.75 öre/kWh
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://www.btea.se/elnat/elnatspriser",
    },
    // NOTE: "PeakHour" (i.e. max per month) will be implemented during Fall of 2025. Today they have "PeakHourPerYear"
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder()
            .fixed_cost(24, 58) // 295 kr/år ÷ 12 = 24.58 kr/månad for standard tariffs
            .fallthrough(true)
            .build()]),
    )),
};
