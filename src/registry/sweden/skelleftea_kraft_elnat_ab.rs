use crate::registry::prelude::*;

pub static SKELLEFTEA_KRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Skellefteå Kraft Elnät AB")
    .vat_number("SE556244395101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.skekraft.se/privat/elnat/elnatspriser/",
                "section",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5420, 0).divide_by(12)),
            (20, Money::new(7815, 0).divide_by(12)),
            (25, Money::new(9480, 0).divide_by(12)),
            (35, Money::new(12340, 0).divide_by(12)),
            (50, Money::new(17315, 0).divide_by(12)),
            (63, Money::new(20965, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(11.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
