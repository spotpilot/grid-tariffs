use crate::registry::prelude::*;

pub static MOLNDAL_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Mölndal Energi Nät AB")
    .vat_number("SE556509458701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.molndalenergi.se/privat/elnat/avgifter-ersattningar")
            .plain_content_locator("main")
            .build(),
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
