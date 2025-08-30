use crate::registry::prelude::*;

pub const PARTILLE_ENERGI: GridOperator = GridOperator {
    name: "Partille Energi",
    vat_number: "SE556528569801",
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fixed_yearly(1875, 0),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::fixed_subunit(30.50),
    other_fees: OtherFees::Unverified,
    links: Links::new("https://partilleenergi.se/elnat/"),
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        CostPeriods::new(&[CostPeriod::builder().load(Base).fixed_cost(32, 50).build()]),
    )),
};
