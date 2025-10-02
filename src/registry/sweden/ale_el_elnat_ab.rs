use crate::registry::prelude::*;

pub static ALE_EL_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ale El Elnät AB")
    .vat_number("SE559398702601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info("https://aleel.se/avgifter/", "main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 6, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(445, 0)),
            (20, Money::new(556, 0)),
            (25, Money::new(728, 0)),
            (35, Money::new(1019, 0)),
            (50, Money::new(1471, 0)),
            (63, Money::new(1841, 0)),
            (80, Money::new(2341, 0)),
            (100, Money::new(2851, 0)),
            (125, Money::new(3594, 0)),
            (160, Money::new(4691, 0)),
            (200, Money::new(6101, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(25.00))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
