use crate::registry::prelude::*;

const FEE_LINK: &str = "https://njudung.se/elnat/elnatsavgifter/";

pub static NJUDUNG_SAVSJO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Njudung Sävsjö Elnät AB")
    .vat_number("SE556659628301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3841, 0).divide_by(12)),
            (20, Money::new(5923, 0).divide_by(12)),
            (25, Money::new(7137, 0).divide_by(12)),
            (35, Money::new(9796, 0).divide_by(12)),
            (50, Money::new(13469, 0).divide_by(12)),
            (63, Money::new(18742, 0).divide_by(12)),
            (80, Money::new(25073, 0).divide_by(12)),
            (100, Money::new(30630, 0).divide_by(12)),
            (125, Money::new(38157, 0).divide_by(12)),
            (160, Money::new(46378, 0).divide_by(12)),
            (200, Money::new(58722, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(36.96))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
