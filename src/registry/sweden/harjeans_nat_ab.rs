use crate::registry::prelude::*;

pub static HARJEANS_NAT_AB: GridOperator = GridOperator::builder()
    .name("Härjeåns Nät AB")
    .vat_number("SE556189319801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.harjeans.se/elnat/elnatspriser-2025/")
            .plain_content_locator(".sectionWrapper")
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
