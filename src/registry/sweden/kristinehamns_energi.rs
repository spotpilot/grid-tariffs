use crate::registry::prelude::*;

pub static KRISTINEHAMNS_ENERGI: GridOperator = GridOperator::builder()
    .name("Kristinehamns Energi")
    .vat_number("SE556526519501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://kristinehamnsenergi.se/elnat/elnatsavgiften/din-elnatsavgift/",
                "section",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fixed(277, 50))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(12.75))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(1),
            CostPeriods::new_all(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(91, 25)
                    .months(November, March)
                    .hours(7, 17)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
                CostPeriod::builder().load(Base).fixed_cost(43, 75).build(),
            ]),
        ))
        .build()])
    .build();
