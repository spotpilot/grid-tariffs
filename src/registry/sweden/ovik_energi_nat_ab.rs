use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.ovikenergi.se/elnat/priser-och-villkor";

pub static OVIK_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Övik Energi Nät AB")
    .vat_number("SE556527706701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://www.ovikenergi.se/elnat/anslut-din-elproduktion")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2024, 10, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3923, 0).divide_by(12)),
            (20, Money::new(6385, 0).divide_by(12)),
            (25, Money::new(8945, 0).divide_by(12)),
            (35, Money::new(11374, 0).divide_by(12)),
            (50, Money::new(15107, 0).divide_by(12)),
            (63, Money::new(19161, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(1.0))
        .transfer_fee(TransferFee::fixed_subunit(11.80))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
