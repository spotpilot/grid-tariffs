use crate::registry::prelude::*;

pub static PARTILLE_ENERGI: GridOperator = GridOperator::builder()
    .name("Partille Energi")
    .vat_number("SE556528569801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info("https://partilleenergi.se/elnat/", "#elnatsavtal")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fixed_yearly(1875, 0))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(7.5))
        .transfer_fee(TransferFee::fixed_subunit(30.50))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(1),
            CostPeriods::new(&[CostPeriod::builder().load(Low).fixed_cost(32, 50).build()]),
        ))
        .build()])
    .build();
