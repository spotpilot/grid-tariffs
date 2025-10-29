use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.nav.se/priser/elnat---priser";

pub static NASSJO_AFFARSVERK_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Nässjö Affärsverk Elnät AB")
    .vat_number("SE556526145901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://www.nav.se/priser/elnat---priser#h-Elproduktion")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2925, 0).divide_by(12)),
            (20, Money::new(4550, 0).divide_by(12)),
            (25, Money::new(5600, 0).divide_by(12)),
            (35, Money::new(7775, 0).divide_by(12)),
            (50, Money::new(10880, 0).divide_by(12)),
            (63, Money::new(13800, 0).divide_by(12)),
            (80, Money::new(17400, 0).divide_by(12)),
            (100, Money::new(20800, 0).divide_by(12)),
            (125, Money::new(24200, 0).divide_by(12)),
            (160, Money::new(29925, 0).divide_by(12)),
            (200, Money::new(36225, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(6.25))
        .transfer_fee(TransferFee::fixed_subunit(27.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
