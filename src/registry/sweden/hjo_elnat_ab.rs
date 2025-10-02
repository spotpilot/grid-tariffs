use crate::registry::prelude::*;

const FEE_LINK: &str = "https://hjoenergi.se/natavgifter/";

const BASE_PRICELIST: PriceListBuilder = PriceListBuilder::new()
    .from_date(2025, 1, 1)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .power_tariff(PowerTariff::NotImplemented);

pub static HJO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Hjo Elnät AB")
    .vat_number("SE559441764301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[
        BASE_PRICELIST
            .variant("Enkeltariff")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(2395, 0).divide_by(12)),
                (20, Money::new(3518, 0).divide_by(12)),
                (25, Money::new(4694, 0).divide_by(12)),
                (35, Money::new(6888, 0).divide_by(12)),
                (50, Money::new(10119, 0).divide_by(12)),
                (63, Money::new(12845, 0).divide_by(12)),
                (80, Money::new(16034, 0).divide_by(12)),
                (100, Money::new(20219, 0).divide_by(12)),
                (125, Money::new(25460, 0).divide_by(12)),
                (160, Money::new(32790, 0).divide_by(12)),
                (200, Money::new(41138, 0).divide_by(12)),
            ]))
            .transfer_fee(TransferFee::fixed_subunit(35.25))
            .build(),
        BASE_PRICELIST
            .variant("Tidstariff")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(3181, 0).divide_by(12)),
                (20, Money::new(4138, 0).divide_by(12)),
                (25, Money::new(5314, 0).divide_by(12)),
                (35, Money::new(7506, 0).divide_by(12)),
                (50, Money::new(10634, 0).divide_by(12)),
                (63, Money::new(13290, 0).divide_by(12)),
                (80, Money::new(16695, 0).divide_by(12)),
                (100, Money::new(20766, 0).divide_by(12)),
                (125, Money::new(25821, 0).divide_by(12)),
                (160, Money::new(32930, 0).divide_by(12)),
                (200, Money::new(41016, 0).divide_by(12)),
            ]))
            .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost_subunit(51.0)
                    .months(November, March)
                    .hours(6, 22)
                    .exclude_weekends()
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost_subunit(18.25)
                    .build(),
            ])))
            .build(),
    ])
    .build();
