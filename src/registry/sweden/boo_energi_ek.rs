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
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(333, 0)),
            (20, Money::new(379, 0)),
            (25, Money::new(440, 0)),
            (35, Money::new(580, 0)),
            (50, Money::new(794, 0)),
            (63, Money::new(980, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(33.00))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
