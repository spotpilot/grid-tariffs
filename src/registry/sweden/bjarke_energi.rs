use crate::registry::prelude::*;

pub const BJÄRKE_ENERGI: GridOperator = GridOperator::builder()
    .name("Bjärke Energi")
    .vat_number("SE763000012801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.bjerke-energi.se/elnat/tariffer/normaltariff/")
            .plain_content_locator("h2 ~ table")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(375, 0)),
            (20, Money::new(437, 50)),
            (25, Money::new(500, 0)),
            (35, Money::new(635, 42)),
            (50, Money::new(833, 33)),
            (63, Money::new(1000, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
            CostPeriod::builder()
                .load(High)
                .fixed_cost_subunit(18.75)
                .months(November, March)
                .hours(6, 22)
                .exclude_weekends_and_swedish_holidays()
                .build(),
            CostPeriod::builder()
                .load(Low)
                .fixed_cost_subunit(16.25)
                .build(),
        ])))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::PeakHour,
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost(50, 0)
                    .months(April, October)
                    .hours(6, 22)
                    .build(),
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost(125, 0)
                    .months(November, March)
                    .hours(6, 22)
                    .build(),
            ]),
        ))
        .build()])
    .build();
