use crate::registry::prelude::*;

const FEE_LINK: &str = "https://ueab.se/elnat/priser/";

pub const ULRICEHAMN_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ulricehamn Energi Elnät AB")
    .vat_number("SE559425365901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3931, 0).divide_by(12)),
            (20, Money::new(5030, 0).divide_by(12)),
            (25, Money::new(6372, 0).divide_by(12)),
            (35, Money::new(9084, 0).divide_by(12)),
            (50, Money::new(13211, 0).divide_by(12)),
            (63, Money::new(16940, 0).divide_by(12)),
            (80, Money::new(23338, 0).divide_by(12)),
            (100, Money::new(29784, 0).divide_by(12)),
            (125, Money::new(37979, 0).divide_by(12)),
            (160, Money::new(49591, 0).divide_by(12)),
            (200, Money::new(63212, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::fixed(163, 00).divide_by(12))
        .feed_in_revenue(FeedInRevenue::fixed_subunit(6.10 - 2.25))
        .transfer_fee(TransferFee::fixed_subunit(28.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
