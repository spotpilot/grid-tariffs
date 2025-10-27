use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.ronneby.se/sidowebbplatser/miljoteknik/elnat/priser-villkor-for-elnat/natavgifter.html";

pub static RONNEBY_MILJOTEKNIK_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ronneby Miljöteknik Elnät AB")
    .vat_number("SE559015198001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[
        PriceList::builder()
            .variant("Tariff 2")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(2008, 0).divide_by(12)),
                (20, Money::new(2741, 0).divide_by(12)),
                (25, Money::new(3760, 0).divide_by(12)),
                (35, Money::new(5822, 0).divide_by(12)),
                (50, Money::new(9054, 0).divide_by(12)),
                (63, Money::new(12349, 0).divide_by(12)),
                (80, Money::new(16381, 0).divide_by(12)),
                (100, Money::new(21358, 0).divide_by(12)),
                (125, Money::new(27542, 0).divide_by(12)),
                (160, Money::new(36302, 0).divide_by(12)),
                (200, Money::new(45568, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::fixed_subunit(51.11))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            .variant("Tariff 9")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(3570, 0).divide_by(12)),
                (20, Money::new(4451, 0).divide_by(12)),
                (25, Money::new(6224, 0).divide_by(12)),
                (35, Money::new(9378, 0).divide_by(12)),
                (50, Money::new(13648, 0).divide_by(12)),
                (63, Money::new(18040, 0).divide_by(12)),
                (80, Money::new(23261, 0).divide_by(12)),
                (100, Money::new(30143, 0).divide_by(12)),
                (125, Money::new(38449, 0).divide_by(12)),
                (160, Money::new(50533, 0).divide_by(12)),
                (200, Money::new(62885, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::new_periods(CostPeriods::new_first(&[
                CostPeriod::builder()
                    .load(High)
                    .fixed_cost_subunit(87.76)
                    .months(November, March)
                    .hours(6, 21)
                    .exclude_weekends()
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost_subunit(17.81)
                    .build(),
            ])))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
    ])
    .build();
