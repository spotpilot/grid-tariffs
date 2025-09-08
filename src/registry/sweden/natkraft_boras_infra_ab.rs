use crate::registry::prelude::*;

pub const NATKRAFT_BORAS_INFRA_AB: GridOperator = GridOperator::builder()
    .name("Nätkraft Borås Infra AB")
    .vat_number("SE556527558201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://natkraftboras.se/elnat/elnatsavtal/priser/")
            .plain_content_locator("main")
            .build(),
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
