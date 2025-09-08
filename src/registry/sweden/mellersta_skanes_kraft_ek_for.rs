use crate::registry::prelude::*;

const FEE_LINK: &str = "https://mskraft.nu/natpriser-2/";

pub const MELLERSTA_SKANES_KRAFT_EK_FOR: GridOperator = GridOperator::builder()
    .name("Mellersta Skånes Kraft, ek för")
    .vat_number("SE737000245801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
