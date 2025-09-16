use crate::{builder, registry::prelude::*};

const FEE_LINK: &str = "https://eksjoenergi.se/elnat/natavgifter/";

pub static EKSJO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Eksjö Elnät AB")
    .vat_number("SE556486766001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("[role='main']")
            .build(),
    ))
    .price_lists(&[
        PriceList::builder()
            .variant("Säkringstariff 11")
            .from_date(2024, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(3185, 0).divide_by(12)),
                (20, Money::new(5238, 0).divide_by(12)),
                (25, Money::new(7041, 0).divide_by(12)),
                (35, Money::new(9920, 0).divide_by(12)),
                (50, Money::new(14860, 0).divide_by(12)),
                (63, Money::new(19842, 0).divide_by(12)),
                (80, Money::new(25844, 0).divide_by(12)),
                (100, Money::new(30828, 0).divide_by(12)),
                (125, Money::new(39679, 0).divide_by(12)),
                (160, Money::new(50768, 0).divide_by(12)),
                (200, Money::new(63331, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::fixed_subunit(23.9))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            .variant("Säkringstariff 42")
            .from_date(2024, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(4633, 0).divide_by(12)),
                (20, Money::new(6486, 0).divide_by(12)),
                (25, Money::new(8479, 0).divide_by(12)),
                (35, Money::new(11355, 0).divide_by(12)),
                (50, Money::new(16279, 0).divide_by(12)),
                (63, Money::new(22432, 0).divide_by(12)),
                (80, Money::new(28490, 0).divide_by(12)),
                (100, Money::new(35459, 0).divide_by(12)),
                (125, Money::new(44440, 0).divide_by(12)),
                (160, Money::new(56656, 0).divide_by(12)),
                (200, Money::new(70707, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(6, 22)
                    .exclude_weekends()
                    .fixed_cost_subunit(47.0)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .fixed_cost_subunit(4.8)
                    .build(),
            ])))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            .variant("Säkringstariff 41")
            .from_date(2024, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(4633, 0).divide_by(12)),
                (20, Money::new(6486, 0).divide_by(12)),
                (25, Money::new(8479, 0).divide_by(12)),
                (35, Money::new(11388, 0).divide_by(12)),
                (50, Money::new(16279, 0).divide_by(12)),
                (63, Money::new(22432, 0).divide_by(12)),
                (80, Money::new(28490, 0).divide_by(12)),
                (100, Money::new(35459, 0).divide_by(12)),
                (125, Money::new(44440, 0).divide_by(12)),
                (160, Money::new(56656, 0).divide_by(12)),
                (200, Money::new(70707, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(6, 22)
                    .exclude_weekends()
                    .fixed_cost_subunit(42.8)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .months(November, March)
                    .fixed_cost_subunit(12.0)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .months(April, October)
                    .fixed_cost_subunit(7.7)
                    .build(),
            ])))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
    ])
    .build();
