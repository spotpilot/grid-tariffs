use crate::registry::prelude::*;

pub static TROLLHATTAN_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Trollhättan Energi Elnät AB")
    .vat_number("SE556686087901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 160))
    .links(
        Links::builder()
            .fee_info("https://www.trollhattanenergi.se/elnat/elnatstaxa/", "main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4055, 0).divide_by(12)),
            (20, Money::new(5035, 0).divide_by(12)),
            (25, Money::new(5990, 0).divide_by(12)),
            (35, Money::new(9470, 0).divide_by(12)),
            (50, Money::new(16420, 0).divide_by(12)),
            (63, Money::new(20660, 0).divide_by(12)),
            (80, Money::new(27465, 0).divide_by(12)),
            (100, Money::new(34200, 0).divide_by(12)),
            (125, Money::new(44135, 0).divide_by(12)),
            (160, Money::new(56285, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(5.60))
        .transfer_fee(TransferFee::fixed_subunit(12.15))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
