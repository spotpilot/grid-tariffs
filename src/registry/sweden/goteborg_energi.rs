use crate::registry::prelude::*;
pub(super) const GÖTEBORG_ENERGI: GridOperator = GridOperator {
    name: "Göteborg Energi",
    price_date: date(2025, 1, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fixed(185, 0),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(25.),
    other_fees: OtherFees::Unverified,
    links: Links {
        fee_info: "https://www.goteborgenergi.se/privat/elnat/elnatsavgiften",
        eltariff_api: Some("https://api.goteborgenergi.cloud/gridtariff/v0/tariffs"),
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDays(3),
        CostPeriods::new(&[CostPeriod::builder().fixed_cost(45, 0).finish()]),
    )),
};
