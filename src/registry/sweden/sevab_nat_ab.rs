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
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
