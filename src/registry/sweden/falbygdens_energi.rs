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
            TariffCalculationMethod::AverageDays(1),
            CostPeriods::new_all(&[
                CostPeriod::builder().load(Base).fixed_cost(39, 80).build(),
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(57, 36)
                    .months(November, March)
                    .hours(7, 19)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
            ]),
        ))
        .build()])
    .build();
