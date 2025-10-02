use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.bengtsforsenergi.se/startsida-nat/sidor/elnatsavgift.html";

pub static BENGTSFORS_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Bengtsfors Energi Nät AB")
    .vat_number("SE556502112701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
