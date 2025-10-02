use crate::registry::prelude::*;

pub static SKANSKA_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Skånska Energi Nät AB")
    .vat_number("SE556497523201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder("https://www.skanska-energi.se/elnat/elnatsavgifter/")
            .plain_content_locator(".content .accordian:first-child")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(6900, 0).divide_by(12)),
            (20, Money::new(9710, 0).divide_by(12)),
            (25, Money::new(12435, 0).divide_by(12)),
            (35, Money::new(17295, 0).divide_by(12)),
            (50, Money::new(24115, 0).divide_by(12)),
            (63, Money::new(30255, 0).divide_by(12)),
            (80, Money::new(39355, 0).divide_by(12)),
            (100, Money::new(48075, 0).divide_by(12)),
            (125, Money::new(62180, 0).divide_by(12)),
            (160, Money::new(79920, 0).divide_by(12)),
            (200, Money::new(103280, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::spot_price_variable(3.3, 0.05, false))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
