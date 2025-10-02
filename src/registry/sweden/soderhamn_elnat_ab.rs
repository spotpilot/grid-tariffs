use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.soderhamnnara.se/sidor/elnat/priser-privatkunder.html";

pub static SODERHAMN_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Söderhamn Elnät AB")
    .vat_number("SE556205999701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 160))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".pagecontent")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3663, 0).divide_by(12)),
            (20, Money::new(4800, 0).divide_by(12)),
            (25, Money::new(6060, 0).divide_by(12)),
            (35, Money::new(8720, 0).divide_by(12)),
            (50, Money::new(12815, 0).divide_by(12)),
            (63, Money::new(16166, 0).divide_by(12)),
            (80, Money::new(21155, 0).divide_by(12)),
            (100, Money::new(27241, 0).divide_by(12)),
            (125, Money::new(35056, 0).divide_by(12)),
            (160, Money::new(46168, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(20.3))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
