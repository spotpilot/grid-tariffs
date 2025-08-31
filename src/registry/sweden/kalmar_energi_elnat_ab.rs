use crate::registry::prelude::*;

pub const KALMAR_ENERGI_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Kalmar Energi Elnät AB")
    .vat_number("SE556182754301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://kalmarenergi.se/foretag/el/elnat/elnatsavgifter/")
            .content_locator_default()
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
