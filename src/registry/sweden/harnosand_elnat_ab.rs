use crate::registry::prelude::*;

pub static HARNOSAND_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Härnösand Elnät AB")
    .vat_number("SE556133332801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            // TODO: This links to a year-specific URL - we need to find another way -
            //       maybe link to start page and find a way to navigate to Elnät > Priser Elnät [2025]
            .fee_info(
                "https://www.hemab.se/elnat/priserelnat2025.4.5382fcb418e57c1263662e9b.html",
                ".pagecontent",
            )
            .feed_in_revenue_info_default(
                "https://www.hemab.se/elnat/priserelnat2025.4.5382fcb418e57c1263662e9b.html",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2024, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3994, 0).divide_by(12)),
            (20, Money::new(6225, 0).divide_by(12)),
            (25, Money::new(8228, 0).divide_by(12)),
            (35, Money::new(12334, 0).divide_by(12)),
            (50, Money::new(17413, 0).divide_by(12)),
            (63, Money::new(21908, 0).divide_by(12)),
            (80, Money::new(27450, 0).divide_by(12)),
            (100, Money::new(33932, 0).divide_by(12)),
            (125, Money::new(41751, 0).divide_by(12)),
            (160, Money::new(53365, 0).divide_by(12)),
            (200, Money::new(65663, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Simple(Cost::Fixed(
            Money::new_subunit(1.75).add_vat(Country::SE),
        )))
        .transfer_fee(TransferFee::fixed_subunit(13.20))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
