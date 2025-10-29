use crate::registry::prelude::*;

pub static MJOLBY_KRAFTNAT_AB: GridOperator = GridOperator::builder()
    .name("Mjölby Kraftnät AB")
    .vat_number("SE556127926501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.mse.se/kundservice/priser-och-avtal/priser-elnat-2025/",
                "#abonnemangspriser",
            )
            .feed_in_revenue_info_default("https://www.mse.se/kundservice/priser-och-avtal/ersattning-avgifter-egen-elproduktion-elnat-2025/")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Simple(Cost::Fixed(
            Money::new_subunit(4.5).add_vat(Country::SE),
        )))
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
