use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.ostrakindselkraft.se/elnatsavgifter/";

pub static OSTRA_KINDS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Östra Kinds Elnät AB")
    .vat_number("SE559461755601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 400))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("[data-elementor-type='wp-page']")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 2, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5600, 0).divide_by(12)),
            (20, Money::new(7000, 0).divide_by(12)),
            (25, Money::new(8750, 0).divide_by(12)),
            (35, Money::new(12250, 0).divide_by(12)),
            (50, Money::new(17500, 0).divide_by(12)),
            (63, Money::new(22050, 0).divide_by(12)),
            (80, Money::new(28000, 0).divide_by(12)),
            (100, Money::new(35000, 0).divide_by(12)),
            (125, Money::new(43750, 0).divide_by(12)),
            (160, Money::new(56000, 0).divide_by(12)),
            (200, Money::new(70000, 0).divide_by(12)),
            (250, Money::new(87500, 0).divide_by(12)),
            (315, Money::new(110250, 0).divide_by(12)),
            (400, Money::new(140000, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fuse_range(&[
            (16, 25, Money::new_subunit(31.25)),
            (35, 400, Money::new_subunit(23.75)),
        ])))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
