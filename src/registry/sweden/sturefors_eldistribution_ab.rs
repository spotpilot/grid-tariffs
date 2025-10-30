use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.stureforsgods.se/elnatinfo";

pub static STUREFORS_ELDISTRIBUTION_AB: GridOperator = GridOperator::builder()
    .name("Sturefors Eldistribution AB")
    .vat_number("SE556528175401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 400))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://www.stureforsgods.se/elnatinfo")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 9, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4797, 0).divide_by(12)),
            (20, Money::new(6061, 0).divide_by(12)),
            (25, Money::new(7141, 0).divide_by(12)),
            (35, Money::new(10385, 0).divide_by(12)),
            (50, Money::new(13878, 0).divide_by(12)),
            (63, Money::new(25946, 0).divide_by(12)),
            (80, Money::new(32733, 0).divide_by(12)),
            (100, Money::new(40662, 0).divide_by(12)),
            (125, Money::new(50608, 0).divide_by(12)),
            (160, Money::new(64588, 0).divide_by(12)),
            (200, Money::new(80514, 0).divide_by(12)),
            (400, Money::new(117364, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Simple(Cost::Fixed(
            Money::new_subunit(3.5).add_vat(Country::SE),
        )))
        .transfer_fee(TransferFee::fixed_subunit(28.35 * 1.25))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
