use crate::registry::prelude::*;

pub static HARJEANS_NAT_AB: GridOperator = GridOperator::builder()
    .name("Härjeåns Nät AB")
    .vat_number("SE556189319801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder("https://www.harjeans.se/elnat/elnatspriser-2025/")
            .plain_content_locator(".sectionWrapper")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(8996, 0).divide_by(12)),
            (20, Money::new(12545, 0).divide_by(12)),
            (25, Money::new(13761, 0).divide_by(12)),
            (35, Money::new(20804, 0).divide_by(12)),
            (50, Money::new(27693, 0).divide_by(12)),
            (63, Money::new(31668, 0).divide_by(12)),
            (80, Money::new(44454, 0).divide_by(12)),
            (100, Money::new(54265, 0).divide_by(12)),
            (125, Money::new(69000, 0).divide_by(12)),
            (160, Money::new(89749, 0).divide_by(12)),
            (200, Money::new(114281, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(6.25))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
