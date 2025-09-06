use crate::registry::prelude::*;

pub const SEVAB_NÄT_AB: GridOperator = GridOperator::builder()
    .name("SEVAB Nät AB")
    .vat_number("SE556192285601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.sevab.com/foretag/elnat/priser-och-avgifter/elnatsavgift-2025")
            .plain_content_locator(".article-content")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
