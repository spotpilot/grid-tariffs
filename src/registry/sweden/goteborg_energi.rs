use crate::registry::prelude::*;
pub const GÖTEBORG_ENERGI: GridOperator = GridOperator {
    name: "Göteborg Energi",
    vat_number: "SE556379272901",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fixed(185, 0),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(25.),
    other_fees: OtherFees::Unverified,
    links: Links::builder()
        .fee_info(
            Link::builder("https://www.goteborgenergi.se/privat/elnat/elnatsavgiften")
                .plain_content_locator("#prisvilla + *")
                .build(),
        )
        .eltariff_api("https://api.goteborgenergi.cloud/gridtariff/v0/tariffs")
        .build(),
    power_tariff: PowerTariff::new(
        TariffCalculationMethod::AverageDays(3),
        CostPeriods::new(&[CostPeriod::builder().load(Base).fixed_cost(45, 0).build()]),
    ),
};
