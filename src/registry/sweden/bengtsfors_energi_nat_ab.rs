use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.bengtsforsenergi.se/startsida-nat/sidor/elnatsavgift.html";

pub static BENGTSFORS_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Bengtsfors Energi Nät AB")
    .vat_number("SE556502112701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3806, 0).divide_by(12)),
            (20, Money::new(4948, 0).divide_by(12)),
            (25, Money::new(6740, 0).divide_by(12)),
            (35, Money::new(9125, 0).divide_by(12)),
            (50, Money::new(11885, 0).divide_by(12)),
            (63, Money::new(15488, 0).divide_by(12)),
            (80, Money::new(19659, 0).divide_by(12)),
            (100, Money::new(21024, 0).divide_by(12)),
            (125, Money::new(24722, 0).divide_by(12)),
            (160, Money::new(32033, 0).divide_by(12)),
            (200, Money::new(40316, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(31.5))
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
