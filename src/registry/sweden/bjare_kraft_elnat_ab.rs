use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.bjarekraft.se/privat/elnat/elnatspriser";

pub static BJARE_KRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Bjäre Kraft Elnät AB")
    .vat_number("SE559427724501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info(FEE_LINK, ".news-article")
    .feed_in_revenue_info_default("https://www.bjarekraft.se/privat/elnat/producera-egen-el/saelj-ditt-oeverskott-av-solel")
    .build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 2, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(355, 0)),
            (20, Money::new(565, 0)),
            (25, Money::new(565, 0)),
            (35, Money::new(1295, 0)),
            (50, Money::new(1984, 0)),
            (63, Money::new(2761, 0)),
            (80, Money::new(3584, 0)),
            (100, Money::new(4703, 0)),
            (125, Money::new(6096, 0)),
            (160, Money::new(8029, 0)),
            (200, Money::new(9866, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(5.3))
        .transfer_fee(TransferFee::fixed_subunit(19.3))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
