use crate::registry::prelude::*;

pub static KARLSTADS_ENERGI: GridOperator = GridOperator::builder()
    .name("Karlstads Energi")
    .vat_number("SE556527673901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, u16::MAX))
    .links(
        Links::builder()
            .fee_info(
                "https://karlstadsenergi.se/gemensamt-innehall/elnat/priser/elnatsavgifter",
                "table",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuse_range(&[
            (16, 63, Money::new(145, 83)),
            (63, u16::MAX, Money::new(642, 19)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fixed_subunit(9.75)))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::PeakHour,
            // NOTE: The pricing is actually the same for 16-63A as >63A if you account for VAT vs without VAT
            CostPeriods::new(&[
                CostPeriod::builder().load(Base).fixed_cost(43, 55).build(),
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(74, 15)
                    .hours(6, 18)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
            ]),
        ))
        .build()])
    .build();
