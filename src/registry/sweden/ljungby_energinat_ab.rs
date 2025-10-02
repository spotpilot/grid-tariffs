use crate::registry::prelude::*;

const FEE_LINK: &str = "https://ljungby-energi.se/elnat/elnatsavgifter/";

pub static LJUNGBY_ENERGINAT_AB: GridOperator = GridOperator::builder()
    .name("Ljungby Energinät AB")
    .vat_number("SE556082399801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
