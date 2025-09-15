use crate::registry::prelude::*;

const FEE_LINK: &str = "https://aselekraft.net/priser/natpriser/";

pub static ASELE_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Åsele Elnät AB")
    .vat_number("SE559012889701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2024, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4298, 0).divide_by(12)),
            (20, Money::new(5651, 0).divide_by(12)),
            (25, Money::new(7090, 0).divide_by(12)),
            (35, Money::new(9966, 0).divide_by(12)),
            (50, Money::new(14236, 0).divide_by(12)),
            (63, Money::new(17925, 0).divide_by(12)),
            (80, Money::new(22786, 0).divide_by(12)),
            (100, Money::new(29646, 0).divide_by(12)),
            (125, Money::new(37111, 0).divide_by(12)),
            (160, Money::new(47614, 0).divide_by(12)),
            (200, Money::new(59593, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(32.5))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
