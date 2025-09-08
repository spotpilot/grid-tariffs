use crate::registry::prelude::*;

pub const TRELLEBORG_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Trelleborg Elnät AB")
    .vat_number("SE559181616901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://trelleborgsenergi.se/el-som-gor-skillnad/vara-elpriser/")
            .plain_content_locator("main")
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
