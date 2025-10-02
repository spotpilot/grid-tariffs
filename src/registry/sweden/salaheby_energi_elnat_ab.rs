use crate::registry::prelude::*;

const FEE_LINK: &str = "https://sheab.se/elnat/natpriser-och-villkor/";

pub static SALAHEBY_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("SalaHeby Energi Elnät AB")
    .vat_number("SE556181367501")
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
