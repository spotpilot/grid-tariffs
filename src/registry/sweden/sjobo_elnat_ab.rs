use crate::registry::prelude::*;

const FEE_LINK: &str = "https://sjoboelnat.se/priser-villkor/";

pub static SJOBO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Sjöbo Elnät AB")
    .vat_number("SE556011551001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(7640, 0).divide_by(12)),
            (20, Money::new(10130, 0).divide_by(12)),
            (25, Money::new(12865, 0).divide_by(12)),
            (35, Money::new(19075, 0).divide_by(12)),
            (50, Money::new(27225, 0).divide_by(12)),
            (63, Money::new(34290, 0).divide_by(12)),
            (80, Money::new(45485, 0).divide_by(12)),
            (100, Money::new(57375, 0).divide_by(12)),
            (125, Money::new(72265, 0).divide_by(12)),
            (160, Money::new(93675, 0).divide_by(12)),
            (200, Money::new(124575, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
