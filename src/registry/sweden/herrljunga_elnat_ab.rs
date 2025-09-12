use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.el.herrljunga.se/elnat/avtal-och-priser/";

pub const HERRLJUNGA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Herrljunga Elnät AB")
    .vat_number("SE556525920601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#content")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
