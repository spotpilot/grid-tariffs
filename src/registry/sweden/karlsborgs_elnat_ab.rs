use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.karlsborgsenergi.se/elnat/elnatspriser/";

pub static KARLSBORGS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Karlsborgs Elnät AB")
    .vat_number("SE559434970501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4697, 0).divide_by(12)),
            (20, Money::new(6793, 0).divide_by(12)),
            (25, Money::new(9064, 0).divide_by(12)),
            (35, Money::new(11864, 0).divide_by(12)),
            (50, Money::new(15648, 0).divide_by(12)),
            (63, Money::new(19479, 0).divide_by(12)),
            (80, Money::new(25795, 0).divide_by(12)),
            (100, Money::new(30737, 0).divide_by(12)),
            (125, Money::new(38768, 0).divide_by(12)),
            (160, Money::new(46248, 0).divide_by(12)),
            (200, Money::new(51023, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(22.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
