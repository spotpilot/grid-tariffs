use crate::registry::prelude::*;

pub const ELLEVIO: GridOperator = GridOperator {
    name: "Ellevio",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(365, 0)),
        (20, Money::new(365, 0)),
        (25, Money::new(365, 0)),
        (35, Money::new(915, 0)),
        (50, Money::new(1400, 0)),
        (63, Money::new(2010, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(6.25),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://www.ellevio.se/abonnemang/elnatspriser-privat/",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDays(3),
        CostPeriods::new(&[
            CostPeriod::builder()
                .cost(Cost::None)
                .include_hours(22, 6)
                .divide_kw_by(2)
                .fallthrough(true)
                .build(),
            CostPeriod::builder().cost(Cost::fixed(81, 25)).build(),
        ]),
    )),
};
