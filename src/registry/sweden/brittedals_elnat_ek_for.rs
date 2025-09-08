use crate::registry::prelude::*;

const FEE_LINK: &str = "https://brittedal.se/elnatspriser";

pub const BRITTEDALS_ELNAT_EK_FOR: GridOperator = GridOperator::builder()
    .name("Brittedals Elnät ek för")
    .vat_number("SE737000010601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
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
