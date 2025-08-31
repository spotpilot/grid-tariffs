use crate::registry::prelude::*;

pub const MÖLNDAL_ENERGI_NÄT_AB: GridOperator = GridOperator::builder()
    .name("Mölndal Energi Nät AB")
    .vat_number("SE556509458701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://www.molndalenergi.se/privat/elnat/avgifter-ersattningar")
            .content_locator_default()
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
