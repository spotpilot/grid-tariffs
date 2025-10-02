use crate::{registry::prelude::*, transfer_fee};
const BASE_PRICELIST: PriceListBuilder = PriceListBuilder::new()
    .from_date(2025, 1, 1)
    .monthly_fee(Cost::fuses(&[
        (16, Money::new(5675, 0).divide_by(12)),
        (20, Money::new(7815, 0).divide_by(12)),
        (25, Money::new(10010, 0).divide_by(12)),
        (35, Money::new(14450, 0).divide_by(12)),
        (50, Money::new(20925, 0).divide_by(12)),
        (63, Money::new(28150, 0).divide_by(12)),
        (80, Money::new(39090, 0).divide_by(12)),
        (100, Money::new(49515, 0).divide_by(12)),
        (125, Money::new(62425, 0).divide_by(12)),
        (160, Money::new(79250, 0).divide_by(12)),
        (200, Money::new(100930, 0).divide_by(12)),
    ]))
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .power_tariff(PowerTariff::NotImplemented);

pub static GOTLANDS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Gotlands Elnät AB")
    .vat_number("SE556537472401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info(
                "https://geab.se/elnat/avgift/elnatspriser/",
                r#"[data-elementor-post-type="page"] > section:nth-child(n + 4)"#,
            )
            .build(),
    )
    .price_lists(&[
        BASE_PRICELIST
            .variant("Enekltariff E4")
            .transfer_fee(TransferFee::fixed_subunit(47.50))
            .build(),
        BASE_PRICELIST
            .variant("Tidstariff T4")
            .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost_subunit(57.50)
                    .months(November, March)
                    .hours(6, 22)
                    .exclude_weekends()
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost_subunit(32.50)
                    .build(),
            ])))
            .build(),
    ])
    .build();
