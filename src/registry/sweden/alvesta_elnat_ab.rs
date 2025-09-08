use crate::registry::prelude::*;

const FEE_LINK: &str = "https://alvestaenergi.se/alvesta-elnat/priser/";

pub const ALVESTA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Alvesta Elnät AB")
    .vat_number("SE556525621001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("section")
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
