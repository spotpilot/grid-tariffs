use crate::registry::prelude::*;

pub static JONKOPING_ENERGI: GridOperator = GridOperator::builder()
    .name("Jönköping Energi")
    .vat_number("SE556449757501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://jonkopingenergi.se/privat/elnat/elnat/priser",
                "section",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 9, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(296, 0)),
            (20, Money::new(351, 0)),
            (25, Money::new(420, 0)),
            (35, Money::new(557, 0)),
            (50, Money::new(762, 0)),
            (63, Money::new(941, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(6.08))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDays(2),
            CostPeriods::new_all(&[
                CostPeriod::builder().load(Base).fixed_cost(27, 70).build(),
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fixed(65, 66))
                    .months(November, March)
                    .hours(7, 20)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
            ]),
        ))
        .build()])
    .build();
