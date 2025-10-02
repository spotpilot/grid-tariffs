use crate::registry::prelude::*;

pub static NATKRAFT_BORAS_INFRA_AB: GridOperator = GridOperator::builder()
    .name("Nätkraft Borås Infra AB")
    .vat_number("SE556527558201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info("https://natkraftboras.se/elnat/elnatsavtal/priser/", "main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
