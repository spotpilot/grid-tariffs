use crate::registry::prelude::*;

pub(super) const BRITTEDAL: GridOperator = GridOperator {
    name: "Brittedal",
    price_date: date(2025, 1, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(464, 0)),
        (20, Money::new(648, 0)),
        (25, Money::new(843, 0)),
        (35, Money::new(1262, 0)),
        (50, Money::new(1866, 0)),
        (63, Money::new(2302, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.0),
    transfer_fee: TransferFee::fixed_subunit(18.0),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://brittedal.se/elnatspriser",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDays(3),
        CostPeriods::new(&[CostPeriod::builder().fixed_cost(50, 0).finish()]),
    )),
};
