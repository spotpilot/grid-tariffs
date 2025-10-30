use crate::registry::prelude::*;

pub static SEVAB_NAT_AB: GridOperator = GridOperator::builder()
    .name("SEVAB Nät AB")
    .vat_number("SE556192285601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            // TODO: This link is year-based. Watch out!
            .fee_info(
                "https://www.sevab.com/privat/elnat/priser-och-avgifter/priser-2025",
                ".article-content",
            )
            .feed_in_revenue_info_default(
                "https://www.sevab.com/privat/elnat/priser-och-avgifter/priser-2025",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4675, 0).divide_by(12)),
            (20, Money::new(6140, 0).divide_by(12)),
            (25, Money::new(7450, 0).divide_by(12)),
            (35, Money::new(10110, 0).divide_by(12)),
            (50, Money::new(14220, 0).divide_by(12)),
            (63, Money::new(17760, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Simple(Cost::Fixed(
            Money::new_subunit(4.2).add_vat(Country::SE),
        )))
        .transfer_fee(TransferFee::fixed_subunit(30.50))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
