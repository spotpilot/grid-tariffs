use crate::registry::prelude::*;

pub const BOO_ENERGI_EK: GridOperator = GridOperator::builder()
    .name("Boo Energi ek.")
    .vat_number("SE714000020401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.booenergi.se/elnatspriser/")
            .plain_content_locator(".electricity_grid_charge_area_main")
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
