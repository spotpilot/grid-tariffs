use crate::registry::prelude::*;

pub const GOTLANDS_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Gotlands Elnät AB")
    .vat_number("SE556537472401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://geab.se/elnat/avgift/elnatspriser/")
            .plain_content_locator(
                r#"[data-elementor-post-type="page"] > section:nth-child(n + 4)"#,
            )
            .build(),
    ))
    .power_tariff(PowerTariff::Unverified)
    .build();
