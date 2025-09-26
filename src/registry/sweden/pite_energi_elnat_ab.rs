use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.piteenergi.se/elnat/elnatsavgifter-2025/";

pub static PITE_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Pite Energi Elnät AB")
    .vat_number("SE556245982501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3195, 0).divide_by(12)),
            (20, Money::new(8810, 0).divide_by(12)),
            (25, Money::new(11060, 0).divide_by(12)),
            (35, Money::new(14825, 0).divide_by(12)),
            (50, Money::new(21000, 0).divide_by(12)),
            (63, Money::new(27250, 0).divide_by(12)),
            (80, Money::new(37900, 0).divide_by(12)),
            (100, Money::new(46750, 0).divide_by(12)),
            (125, Money::new(56850, 0).divide_by(12)),
            (160, Money::new(72225, 0).divide_by(12)),
            (200, Money::new(90975, 0).divide_by(12)),
            (250, Money::new(112775, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fuse_range(&[
            (16, 16, Money::new_subunit(18.70)),
            (20, 250, Money::ZERO),
        ])))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
