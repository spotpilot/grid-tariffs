use crate::registry::prelude::*;

const FEE_LINK: &str = "https://hallstavikselverk.se/elnat/elnatspriser/";

pub const HALLSTAVIKS_ELVERK_EK_FOR: GridOperator = GridOperator::builder()
    .name("Hallstaviks Elverk ek för")
    .vat_number("SE714400051501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".gital-page-container")
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
