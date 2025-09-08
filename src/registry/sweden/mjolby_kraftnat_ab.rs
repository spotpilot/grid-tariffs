use crate::registry::prelude::*;

pub const MJOLBY_KRAFTNAT_AB: GridOperator = GridOperator::builder()
    .name("Mjölby Kraftnät AB")
    .vat_number("SE556127926501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.mse.se/kundservice/priser-och-avtal/priser-elnat-2025/")
            .plain_content_locator("#abonnemangspriser")
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
