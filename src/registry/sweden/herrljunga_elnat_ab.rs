use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.el.herrljunga.se/elnat/avtal-och-priser/";

pub static HERRLJUNGA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Herrljunga Elnät AB")
    .vat_number("SE556525920601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 125))
    .links(Links::builder().fee_info(FEE_LINK, "#content").build())
    .price_lists(&[PriceList::builder()
        .from_date(2024, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3125, 0).divide_by(12)),
            (20, Money::new(5000, 0).divide_by(12)),
            (25, Money::new(7250, 0).divide_by(12)),
            (35, Money::new(9750, 0).divide_by(12)),
            (50, Money::new(12375, 0).divide_by(12)),
            (63, Money::new(15000, 0).divide_by(12)),
            (80, Money::new(19500, 0).divide_by(12)),
            (100, Money::new(24500, 0).divide_by(12)),
            (125, Money::new(28250, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(30.60))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
