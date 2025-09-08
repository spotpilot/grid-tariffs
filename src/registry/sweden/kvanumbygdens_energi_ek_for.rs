use crate::registry::prelude::*;

const FEE_LINK: &str = "https://kvanumenergi.se/elnat/natavgift/";

pub const KVANUMBYGDENS_ENERGI_EK_FOR: GridOperator = GridOperator::builder()
    .name("Kvänumbygdens Energi ek. för")
    .vat_number("SE768400219401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#text_block-6-11020")
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
