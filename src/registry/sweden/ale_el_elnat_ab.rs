use crate::registry::prelude::*;

pub const ALE_EL_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Ale El Elnät AB")
    .vat_number("SE559398702601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://aleel.se/avgifter/")
            .plain_content_locator("main")
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
