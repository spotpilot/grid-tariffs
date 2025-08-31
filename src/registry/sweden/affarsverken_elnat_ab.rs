use crate::registry::prelude::*;

pub const AFFÄRSVERKEN_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Affärsverken Elnät AB")
    .vat_number("SE556532083401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://www.affarsverken.se/elnat/elnatspriser/privatkund/")
            .content_locator_default()
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
