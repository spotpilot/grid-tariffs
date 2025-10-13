use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.smedjebackenenergi.se/elnat/elnatspriser.html";

pub static SMEDJEBACKEN_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Smedjebacken Energi Nät AB")
    .vat_number("SE556527871901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4526, 25).divide_by(12)),
            (20, Money::new(4526, 25).divide_by(12)),
            (25, Money::new(6077, 50).divide_by(12)),
            (35, Money::new(7208, 75).divide_by(12)),
            (50, Money::new(91105, 0).divide_by(12)),
            (63, Money::new(10352, 5).divide_by(12)),
            (80, Money::new(11568, 75).divide_by(12)),
            (100, Money::new(18076, 25).divide_by(12)),
            (125, Money::new(23861, 25).divide_by(12)),
            (160, Money::new(28200, 0).divide_by(12)),
            (200, Money::new(38210, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(26.25))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
