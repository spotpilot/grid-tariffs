use crate::registry::prelude::*;

pub const LERUM_NAT_AB: GridOperator = GridOperator::builder()
    .name("Lerum Nät AB")
    .vat_number("SE556109395501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .new_fee_info(
                "https://www.lerumenergi.se/Elnat/Elnatsavgifter",
                "#MainContent",
            )
            .new_feed_in_revenue_info(
                "https://www.lerumenergi.se/Solenergi/Salj-ditt-overskott",
                "#MainContent",
            )
            .build(),
    )
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
