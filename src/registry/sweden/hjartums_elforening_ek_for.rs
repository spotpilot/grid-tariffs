use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://www.hjartumselforening.se/elnat/natariff/sakringstariffer-from-2025-01-01/";

const BASE_PRICELIST: PriceListBuilder = PriceListBuilder::new()
    .from_date(2025, 1, 1)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .power_tariff(PowerTariff::NotImplemented);

pub static HJARTUMS_ELFORENING_EK_FOR: GridOperator = GridOperator::builder()
    .name("Hjärtums Elförening ek för")
    .vat_number("SE758500053701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 500))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[
        BASE_PRICELIST
            .variant("Enkeltariff")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(4311, 0).divide_by(12)),
                (20, Money::new(5474, 0).divide_by(12)),
                (25, Money::new(6764, 0).divide_by(12)),
                (35, Money::new(9410, 0).divide_by(12)),
                (50, Money::new(13381, 0).divide_by(12)),
                (63, Money::new(16845, 0).divide_by(12)),
                (80, Money::new(21538, 0).divide_by(12)),
                (100, Money::new(26738, 0).divide_by(12)),
                (125, Money::new(33798, 0).divide_by(12)),
                (160, Money::new(43449, 0).divide_by(12)),
                (200, Money::new(54111, 0).divide_by(12)),
                (250, Money::new(66469, 0).divide_by(12)),
            ]))
            .transfer_fee(TransferFee::fixed_subunit(24.9))
            .build(),
        BASE_PRICELIST
            .variant("Tidstariff")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(5153, 0).divide_by(12)),
                (20, Money::new(6848, 0).divide_by(12)),
                (25, Money::new(8308, 0).divide_by(12)),
                (35, Money::new(10953, 0).divide_by(12)),
                (50, Money::new(14925, 0).divide_by(12)),
                (63, Money::new(18388, 0).divide_by(12)),
                (80, Money::new(23080, 0).divide_by(12)),
                (100, Money::new(28281, 0).divide_by(12)),
                (125, Money::new(35341, 0).divide_by(12)),
                (160, Money::new(48299, 0).divide_by(12)),
                (200, Money::new(55656, 0).divide_by(12)),
                (250, Money::new(68019, 0).divide_by(12)),
                (315, Money::new(86195, 0).divide_by(12)),
                (400, Money::new(107799, 0).divide_by(12)),
                (500, Money::new(132956, 0).divide_by(12)),
            ]))
            .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost_subunit(37.4)
                    .months(November, March)
                    .hours(6, 21)
                    .exclude_weekends()
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost_subunit(14.9)
                    .build(),
            ])))
            .build(),
    ])
    .build();
