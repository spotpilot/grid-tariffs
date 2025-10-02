use crate::registry::prelude::*;

pub static FALKENBERG_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Falkenberg Energi Elnät AB")
    .vat_number("SE556461483101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.falkenberg-energi.se/elnat/elnatspriser/sakringsabonnemang/",
                ".entry-content",
            )
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
