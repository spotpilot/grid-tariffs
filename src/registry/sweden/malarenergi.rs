use crate::registry::prelude::*;

const BASE: GridOperatorBuilder = GridOperatorBuilder::new()
    .name("Mälarenergi")
    .vat_number("SE556554150401")
    .price_date(2025, 9, 1)
    .country(Country::SE)
    .monthly_production_fee(Cost::Unverified)
    .other_fees(OtherFees::Unverified)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .feed_in_revenue(FeedInRevenue::new_periods(CostPeriods::new(&[
        CostPeriod::builder()
            .load(High)
            .fixed_cost_subunit(9.11)
            .include_months(November, March)
            .include_hours(6, 22)
            .exclude_weekends()
            .build(),
        CostPeriod::builder()
            .load(Low)
            .fixed_cost_subunit(4.91)
            .build(),
    ])))
    .links(Links::new(
        "https://www.malarenergi.se/el/elnat/priser-elnat/",
    ));

pub const MÄLARENERGI: GridOperator = BASE
    .monthly_fee(Cost::fuses(&[
        (16, Money::new(307, 50)),
        (20, Money::new(335, 0)),
        (25, Money::new(362, 50)),
        (35, Money::new(437, 50)),
        (50, Money::new(540, 0)),
        (63, Money::new(625, 0)),
    ]))
    .transfer_fee(TransferFee::fixed_subunit(36.25))
    .power_tariff(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder()
            .load(High)
            .fixed_cost(18, 75)
            .include_hours(7, 19)
            .exclude_weekends_and_swedish_holidays()
            .build()]),
    ))
    .build();

pub const MÄLARENERGI_POWER_BASED: GridOperator = BASE
    .monthly_fee(Cost::fixed(896, 00))
    .transfer_fee(TransferFee::fixed(0, 09))
    .power_tariff(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder()
            .load(High)
            .cost(Cost::fixed(69, 68).add_vat(Country::SE))
            .include_hours(7, 19)
            .exclude_weekends_and_swedish_holidays()
            .build()]),
    ))
    .build();
