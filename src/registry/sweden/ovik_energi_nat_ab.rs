use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.ovikenergi.se/elnat/priser-och-villkor";

pub const OVIK_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Övik Energi Nät AB")
    .vat_number("SE556527706701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
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
