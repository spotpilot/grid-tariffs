use crate::registry::prelude::*;

pub const UDDEVALLA_ENERGI_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Uddevalla Energi Elnät AB")
    .vat_number("SE556762562801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.uddevallaenergi.se/privat/elnat.html")
            .plain_content_locator("div:has(> #Priserochavtal) + div")
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
