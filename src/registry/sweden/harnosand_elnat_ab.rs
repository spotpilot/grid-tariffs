use crate::registry::prelude::*;

pub const HARNOSAND_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Härnösand Elnät AB")
    .vat_number("SE556133332801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        // TODO: This links to a year-specific URL - we need to find another way -
        //       maybe link to start page and find a way to navigate to Elnät > Priser Elnät [2025]
        Link::builder("https://www.hemab.se/elnat/priserelnat2025.4.5382fcb418e57c1263662e9b.html")
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
