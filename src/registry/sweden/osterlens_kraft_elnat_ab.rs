use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://www.osterlenskraft.se/elnat/arsavgift-for-sakringskunder-fran-1-januari-2025/";

pub static OSTERLENS_KRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Österlens Kraft Elnät AB")
    .vat_number("SE556406305401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#breadcrumb + .bde-section")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(6155, 0).divide_by(12)),
            (20, Money::new(7710, 0).divide_by(12)),
            (25, Money::new(9511, 0).divide_by(12)),
            (35, Money::new(13186, 0).divide_by(12)),
            (50, Money::new(18671, 0).divide_by(12)),
            (63, Money::new(23469, 0).divide_by(12)),
            (80, Money::new(29741, 0).divide_by(12)),
            (100, Money::new(37134, 0).divide_by(12)),
            (125, Money::new(46303, 0).divide_by(12)),
            (160, Money::new(59195, 0).divide_by(12)),
            (200, Money::new(73906, 0).divide_by(12)),
            (250, Money::new(92249, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
