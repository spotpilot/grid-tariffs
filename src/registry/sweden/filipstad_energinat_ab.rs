use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.filipstadenerginat.se/forkunder/avgifterochvillkor.64.html";

pub const FILIPSTAD_ENERGINAT_AB: GridOperator = GridOperator::builder()
    .name("Filipstad Energinät AB")
    .vat_number("SE556517499101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".pagecontent")
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
