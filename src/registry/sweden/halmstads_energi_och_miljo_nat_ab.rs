use crate::registry::prelude::*;

pub static HALMSTADS_ENERGI_OCH_MILJO_NAT_AB: GridOperator = GridOperator::builder()
    .name("Halmstads Energi och Miljö Nät AB")
    .vat_number("SE556330398001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.hem.se/elnat/avtal-och-priser",
                "#sektion-Sektion",
            )
            .feed_in_revenue_info_default("https://www.hem.se/elnat/avtal-och-priser")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4290, 0).divide_by(12)),
            (20, Money::new(5591, 0).divide_by(12)),
            (25, Money::new(7454, 0).divide_by(12)),
            (35, Money::new(11676, 0).divide_by(12)),
            (50, Money::new(17592, 0).divide_by(12)),
            (63, Money::new(22767, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(6.25))
        .transfer_fee(TransferFee::fixed_subunit(9.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
