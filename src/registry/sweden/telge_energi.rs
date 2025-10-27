use std::marker::PhantomData;

use crate::registry::prelude::*;

pub static TELGE_ENERGI: GridOperator = GridOperator::builder()
    .name("Telge Energi")
    .vat_number("SE559463499901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.telge.se/elnat/elnatskostnad/elnatspriser/",
                "main",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(415, 0)),
            (20, Money::new(451, 25)),
            (25, Money::new(496, 58)),
            (35, Money::new(587, 33)),
            (50, Money::new(723, 42)),
            (63, Money::new(841, 33)),
        ]))
        .transfer_fee(TransferFee::fixed_subunit(7.0))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Simple(Cost::fixed_subunit(5.60)))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(1),
            CostPeriods::new_first(&[CostPeriod::builder()
                .load(High)
                .fixed_cost(28, 0)
                .months(November, March)
                .hours(7, 20)
                .exclude_weekends()
                .exclude_holidays(Country::SE)
                .build()]),
        ))
        .build()])
    .build();
