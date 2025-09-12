use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.hedemoraenergi.se/el/kopa-el/elnat/avgifter-och-prislista/";

pub const HEDEMORA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Hedemora Energi AB")
    .vat_number("SE556115522601")
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
