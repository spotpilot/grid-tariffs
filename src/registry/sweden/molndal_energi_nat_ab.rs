use crate::registry::prelude::*;

pub static MOLNDAL_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Mölndal Energi Nät AB")
    .vat_number("SE556509458701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 35))
    .links(Links::new(
        Link::builder("https://www.molndalenergi.se/privat/elnat/avgifter-ersattningar")
            .plain_content_locator("main")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 9, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3108, 0).divide_by(12)),
            (20, Money::new(4879, 0).divide_by(12)),
            (25, Money::new(4879, 0).divide_by(12)),
            (35, Money::new(7908, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(5.36))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDays(3),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(7, 18)
                    .exclude_weekends_and_swedish_holidays()
                    .fixed_cost(100, 79)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .cost(Cost::fixed(0, 0))
                    .build(),
            ]),
        ))
        .build()])
    .build();
