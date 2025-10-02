use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://www.skurupselverk.se/elnat/priserochavtal.4.4288834418e65aed20f7c8b.html";

pub static SKURUPS_ELVERK_AB: GridOperator = GridOperator::builder()
    .name("Skurups Elverk AB")
    .vat_number("SE556934168701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2024, 9, 26)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4259, 0).divide_by(12)),
            (20, Money::new(7434, 0).divide_by(12)),
            (25, Money::new(9449, 0).divide_by(12)),
            (35, Money::new(13783, 0).divide_by(12)),
            (50, Money::new(19800, 0).divide_by(12)),
            (63, Money::new(25993, 0).divide_by(12)),
            (80, Money::new(35980, 0).divide_by(12)),
            (100, Money::new(45245, 0).divide_by(12)),
            (125, Money::new(58652, 0).divide_by(12)),
            (160, Money::new(75523, 0).divide_by(12)),
            (200, Money::new(94254, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(34.80))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
