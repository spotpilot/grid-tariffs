use crate::registry::prelude::*;
pub static GOTEBORG_ENERGI: GridOperator = GridOperator::builder()
    .name("Göteborg Energi")
    .vat_number("SE556379272901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                Link::builder("https://www.goteborgenergi.se/privat/elnat/elnatsavgiften")
                    .plain_content_locator("#prisvilla + *")
                    .build(),
            )
            .eltariff_api("https://api.goteborgenergi.cloud/gridtariff/v0/tariffs")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fixed(185, 0))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(25.))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDays(3),
            CostPeriods::new(&[CostPeriod::builder().load(Base).fixed_cost(45, 0).build()]),
        ))
        .build()])
    .build();
