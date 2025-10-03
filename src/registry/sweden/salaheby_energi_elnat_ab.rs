use crate::registry::prelude::*;

const FEE_LINK: &str = "https://sheab.se/elnat/natpriser-och-villkor/";

pub static SALAHEBY_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("SalaHeby Energi Elnät AB")
    .vat_number("SE556181367501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(1600, 0).divide_by(12)),
            (20, Money::new(2500, 0).divide_by(12)),
            (25, Money::new(3250, 0).divide_by(12)),
            (35, Money::new(5950, 0).divide_by(12)),
            (50, Money::new(8500, 0).divide_by(12)),
            (63, Money::new(10710, 0).divide_by(12)),
            (80, Money::new(14400, 0).divide_by(12)),
            (100, Money::new(18000, 0).divide_by(12)),
            (125, Money::new(22500, 0).divide_by(12)),
            (160, Money::new(28800, 0).divide_by(12)),
            (200, Money::new(36000, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fuse_range(&[
                        (16, 25, Money::new(135, 0)),
                        (35, 63, Money::new(132, 0)),
                        (80, 200, Money::new(130, 0)),
                    ]))
                    .hours(7, 19)
                    .months(November, March)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .cost(Cost::fuse_range(&[
                        (16, 25, Money::new(56, 0)),
                        (35, 63, Money::new(52, 0)),
                        (80, 200, Money::new(43, 0)),
                    ]))
                    .hours(7, 19)
                    .months(April, October)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
            ]),
        ))
        .build()])
    .build();
