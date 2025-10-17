use crate::registry::prelude::*;

pub static KARLSKOGA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Karlskoga Elnät AB")
    .vat_number("SE556507429001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.karlskogaenergi.se/Vara-tjanster/elnat/priser-och-avtalsvillkor/",
                "#mainContent",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2375, 0).divide_by(12)),
            (20, Money::new(3438, 0).divide_by(12)),
            (25, Money::new(5875, 0).divide_by(12)),
            (35, Money::new(8355, 0).divide_by(12)),
            (50, Money::new(11805, 0).divide_by(12)),
            (63, Money::new(15299, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(18.90))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::PeakHour,
            CostPeriods::new(&[
                CostPeriod::builder().load(Base).fixed_cost(51, 45).build(),
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(87, 54)
                    .months(November, March)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    // https://www.karlskogaenergi.se/Vara-tjanster/elnat/fragor--svar/
                    // "mellan klockan 06.00-18.00"
                    .hours(6, 17)
                    .build(),
            ]),
        ))
        .build()])
    .build();
