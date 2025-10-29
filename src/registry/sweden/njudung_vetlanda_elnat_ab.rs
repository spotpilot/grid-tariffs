use crate::registry::prelude::*;

const FEE_LINK: &str = "https://njudung.se/elnat/elnatsavgifter/";

pub static NJUDUNG_VETLANDA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Njudung Vetlanda Elnät AB")
    .vat_number("SE556819474901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://njudung.se/elnat/elnatsavgifter/")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2744, 0).divide_by(12)),
            (20, Money::new(4093, 0).divide_by(12)),
            (25, Money::new(5168, 0).divide_by(12)),
            (35, Money::new(7236, 0).divide_by(12)),
            (50, Money::new(10370, 0).divide_by(12)),
            (63, Money::new(13582, 0).divide_by(12)),
            (80, Money::new(17296, 0).divide_by(12)),
            (100, Money::new(21569, 0).divide_by(12)),
            (125, Money::new(28362, 0).divide_by(12)),
            (160, Money::new(36240, 0).divide_by(12)),
            (200, Money::new(45302, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(6.25))
        .transfer_fee(TransferFee::fixed_subunit(33.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
