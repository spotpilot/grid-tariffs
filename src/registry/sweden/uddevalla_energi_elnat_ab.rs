use crate::registry::prelude::*;

pub static UDDEVALLA_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Uddevalla Energi Elnät AB")
    .vat_number("SE556762562801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.uddevallaenergi.se/privat/elnat.html")
            .plain_content_locator("div:has(> #Priserochavtal) + div")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3750, 0).divide_by(12)),
            (20, Money::new(5675, 0).divide_by(12)),
            (25, Money::new(8000, 0).divide_by(12)),
            (35, Money::new(11500, 0).divide_by(12)),
            (50, Money::new(16250, 0).divide_by(12)),
            (63, Money::new(21250, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(15.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
