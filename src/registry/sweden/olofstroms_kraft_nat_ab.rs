use crate::registry::prelude::*;

pub const OLOFSTROMS_KRAFT_NAT_AB: GridOperator = GridOperator::builder()
    .name("Olofströms Kraft Nät AB")
    .vat_number("SE556462112501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.olofstromskraft.se/natavgifter-el")
            .plain_content_locator(".main-content")
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
