use crate::registry::prelude::*;

pub static DALA_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Dala Energi Elnät AB")
    .vat_number("SE556166775801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info("https://dalaenergi.se/el/avgift/", "*:has(> #priser)")
            .feed_in_revenue_info_default("https://dalaenergi.se/el/avgift/")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 4, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4100, 0).divide_by(12)),
            (20, Money::new(5750, 0).divide_by(12)),
            (25, Money::new(7200, 0).divide_by(12)),
            (35, Money::new(9950, 0).divide_by(12)),
            (50, Money::new(14250, 0).divide_by(12)),
            (63, Money::new(18000, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(7.5))
        .transfer_fee(TransferFee::fixed_subunit(8.0))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(7, 19)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .fixed_cost(95, 0)
                    .build(),
                CostPeriod::builder()
                    .load(High)
                    .months(April, October)
                    .hours(7, 19)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .fixed_cost(35, 0)
                    .build(),
                CostPeriod::builder().load(Low).fixed_cost(35, 0).build(),
            ]),
        ))
        .build()])
    .build();
