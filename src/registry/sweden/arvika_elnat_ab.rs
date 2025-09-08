use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.teknikivast.se/sidor/elnat/ditt-elnatsabonnemang/din-natavgift/natavgift-sakringskund.html";

pub const ARVIKA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Arvika Elnät AB")
    .vat_number("SE556527671301")
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
