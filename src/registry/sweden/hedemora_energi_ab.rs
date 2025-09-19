use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.hedemoraenergi.se/el/kopa-el/elnat/avgifter-och-prislista/";

pub static HEDEMORA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Hedemora Energi AB")
    .vat_number("SE556115522601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 500))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3352, 0).divide_by(12)),
            (20, Money::new(4367, 0).divide_by(12)),
            (25, Money::new(5429, 0).divide_by(12)),
            (35, Money::new(6854, 0).divide_by(12)),
            (50, Money::new(9029, 0).divide_by(12)),
            (63, Money::new(11577, 0).divide_by(12)),
            (80, Money::new(14615, 0).divide_by(12)),
            (100, Money::new(18527, 0).divide_by(12)),
            (125, Money::new(24988, 0).divide_by(12)),
            (160, Money::new(34305, 0).divide_by(12)),
            (200, Money::new(42889, 0).divide_by(12)),
            (225, Money::new(50337, 0).divide_by(12)),
            (250, Money::new(56031, 0).divide_by(12)),
            (315, Money::new(66427, 0).divide_by(12)),
            (355, Money::new(74378, 0).divide_by(12)),
            (400, Money::new(83587, 0).divide_by(12)),
            (500, Money::new(104175, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(29.56))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
