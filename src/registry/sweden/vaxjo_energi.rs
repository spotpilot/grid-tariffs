use crate::registry::prelude::*;

pub const VÄXJÖ_ENERGI: GridOperator = GridOperator {
    name: "Växjö Energi",
    vat_number: "SE556526851201",
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(248, 75)),
        (20, Money::new(400, 00)),
        (25, Money::new(486, 25)),
        (35, Money::new(660, 00)),
        (50, Money::new(920, 00)),
        (63, Money::new(1145, 00)),
        (80, Money::new(1607, 50)),
        (100, Money::new(1962, 50)),
        (125, Money::new(2386, 25)),
        (160, Money::new(2992, 50)),
        (200, Money::new(3685, 00)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::SpotPriceVariable {
        base_cost: Cost::fixed_subunit(1.48),
        spot_price_multiplier: 0.0881,
        approximated: false,
    },
    transfer_fee: TransferFee::SpotPriceVariable {
        base_cost: Cost::fixed_subunit(8.86),
        spot_price_multiplier: 0.1126,
        approximated: false,
    },
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 1000),
    other_fees: OtherFees::Unverified,
    links: Links::new("https://www.veab.se/privat/elnat/elnatsavgift/"),
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageHours(3),
        CostPeriods::new(&[CostPeriod::builder()
            .load(High)
            .cost(Cost::fuse_range(&[
                (16, 63, Money::new(65, 48)),
                (80, 1000, Money::new(91, 44)),
            ]))
            .include_hours(7, 20)
            .exclude_weekends_and_swedish_holidays()
            .build()]),
    )),
};
