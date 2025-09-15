use crate::registry::prelude::*;

const FEE_LINK: &str = "https://alvestaenergi.se/alvesta-elnat/priser/";

pub static ALVESTA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Alvesta Elnät AB")
    .vat_number("SE556525621001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("section.ae-table,section.ae-article")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3572, 0).divide_by(12)),
            (20, Money::new(5483, 0).divide_by(12)),
            (25, Money::new(7242, 0).divide_by(12)),
            (35, Money::new(11423, 0).divide_by(12)),
            (50, Money::new(16890, 0).divide_by(12)),
            (63, Money::new(23245, 0).divide_by(12)),
            (80, Money::new(31318, 0).divide_by(12)),
            (100, Money::new(41302, 0).divide_by(12)),
            (125, Money::new(51625, 0).divide_by(12)),
            (160, Money::new(64873, 0).divide_by(12)),
            (200, Money::new(80473, 0).divide_by(12)),
            (250, Money::new(98673, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(36.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
