use crate::registry::prelude::*;

const FEE_LINK: &str = "https://tidaholmsenergi.se/elnat/natpriser/";

pub static TIDAHOLMS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Tidaholms Elnät AB")
    .vat_number("SE556004333201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#content")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2345, 0).divide_by(12)),
            (20, Money::new(3750, 0).divide_by(12)),
            (25, Money::new(4690, 0).divide_by(12)),
            (35, Money::new(6485, 0).divide_by(12)),
            (50, Money::new(9250, 0).divide_by(12)),
            (63, Money::new(11665, 0).divide_by(12)),
            (80, Money::new(14755, 0).divide_by(12)),
            (100, Money::new(18325, 0).divide_by(12)),
            (125, Money::new(23260, 0).divide_by(12)),
            (160, Money::new(29430, 0).divide_by(12)),
            (200, Money::new(37075, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(30.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
