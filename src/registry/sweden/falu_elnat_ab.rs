use crate::registry::prelude::*;

pub static FALU_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Falu Elnät AB")
    .vat_number("SE556002847301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://fev.se/el/elnat/elnatspriser-privat.html",
                ".pagecontent",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2024, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(400, 0)),
            (20, Money::new(520, 0)),
            (25, Money::new(650, 0)),
            (35, Money::new(890, 0)),
            (50, Money::new(1235, 0)),
            (63, Money::new(1565, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(15.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
