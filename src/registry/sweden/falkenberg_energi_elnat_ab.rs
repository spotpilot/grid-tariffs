use crate::registry::prelude::*;

pub const FALKENBERG_ENERGI_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Falkenberg Energi Elnät AB")
    .vat_number("SE556461483101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://www.falkenberg-energi.se/elnat/elnatspriser/sakringsabonnemang/")
            .plain_content_locator(".entry-content")
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
