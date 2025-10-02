use crate::registry::prelude::*;

const FEE_LINK: &str = "https://mskraft.nu/natpriser-t/";

pub static MELLERSTA_SKANES_KRAFT_EK_FOR: GridOperator = GridOperator::builder()
    .name("Mellersta Skånes Kraft, ek för")
    .vat_number("SE737000245801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(8942, 0).divide_by(12)),
            (20, Money::new(12140, 0).divide_by(12)),
            (25, Money::new(15112, 0).divide_by(12)),
            (35, Money::new(19268, 0).divide_by(12)),
            (50, Money::new(24960, 0).divide_by(12)),
            (63, Money::new(32440, 0).divide_by(12)),
            (80, Money::new(40170, 0).divide_by(12)),
            (100, Money::new(52184, 0).divide_by(12)),
            (125, Money::new(67900, 0).divide_by(12)),
            (160, Money::new(94140, 0).divide_by(12)),
            (200, Money::new(118620, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(25.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
