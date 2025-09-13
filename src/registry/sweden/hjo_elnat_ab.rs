use crate::registry::prelude::*;

const FEE_LINK: &str = "https://hjoenergi.se/natavgifter/";

pub static HJO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Hjo Elnät AB")
    .vat_number("SE559441764301")
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
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
