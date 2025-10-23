use crate::registry::prelude::*;

pub static FALBYGDENS_ENERGI: GridOperator = GridOperator::builder()
    .name("Falbygdens Energi")
    .vat_number("SE556407516501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://falbygdensenergi.se/privat/vart-elnat/elnatsavgifter-samt-villkor",
                ".pagecontent",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fixed(343, 42))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(37.2))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDaysDifferentiated { base: 1, peak: 1 },
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    // Includes base cost 39,80
                    .fixed_cost(97, 16)
                    .months(November, March)
                    .hours(7, 19)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
                CostPeriod::builder().load(Low).fixed_cost(39, 80).build(),
            ]),
        ))
        .build()])
    .build();
