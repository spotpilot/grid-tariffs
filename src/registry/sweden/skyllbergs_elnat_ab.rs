use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.skyllbergsbruk.se/elnat";

pub static SKYLLBERGS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Skyllbergs Elnät AB")
    .vat_number("SE559440600001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("[id='1230953390']")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(8570, 0).divide_by(12)),
            (20, Money::new(10819, 0).divide_by(12)),
            (25, Money::new(14623, 0).divide_by(12)),
            (35, Money::new(20548, 0).divide_by(12)),
            (50, Money::new(29193, 0).divide_by(12)),
            (63, Money::new(37022, 0).divide_by(12)),
            (80, Money::new(46926, 0).divide_by(12)),
            (100, Money::new(58603, 0).divide_by(12)),
            (125, Money::new(73236, 0).divide_by(12)),
            (160, Money::new(95407, 0).divide_by(12)),
            (200, Money::new(114926, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(45.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
