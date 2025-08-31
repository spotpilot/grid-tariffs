use crate::{builder::GridOperatorBuilder, registry::prelude::*};

const BASE: GridOperatorBuilder = GridOperator::builder()
    .vat_number("SE556070606001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    // 92,5% correlation between spot price and grid benefit from my own finding
    .feed_in_revenue(FeedInRevenue::SpotPriceVariable {
        base_cost: Cost::fixed_subunit(4.58),
        spot_price_multiplier: 0.06,
        approximated: true,
    })
    .monthly_production_fee(Cost::Unverified)
    .other_fees(OtherFees::Unverified)
    .power_tariff(PowerTariff::NotImplemented)
    .links(Links::new(
        Link::builder("https://www.eon.se/el/elnat/elnaetsabonnemang-priser")
            .content_locator(ContentLocator::new(
                CssSelector(r#"eon-ui-table-renderer"#),
                ContentTarget::Attribute("content"),
            ))
            .build(),
    ));

pub const SYD: GridOperator = BASE
    .name("E.ON Syd")
    .price_date(2024, 8, 1)
    .monthly_fee(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(215, 0)),
        (16, 16, None, Money::new(591, 25)),
        (20, 20, None, Money::new(780, 0)),
        (25, 25, None, Money::new(1006, 25)),
        (35, 35, None, Money::new(1472, 50)),
        (50, 50, None, Money::new(2203, 75)),
        (63, 63, None, Money::new(2865, 0)),
    ]))
    .transfer_fee(TransferFee::Simple(
        Cost::fuse_range_with_yearly_consumption(&[
            (16, 16, Some(8000), Money::new_subunit(87.15)),
            (16, 63, None, Money::new_subunit(30.70)),
        ]),
    ))
    .build();

pub const STOCKHOLM: GridOperator = BASE
    .name("E.ON Stockholm")
    .price_date(2025, 7, 1)
    .monthly_fee(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(215, 00)),
        (16, 16, None, Money::new(473, 75)),
        (20, 20, None, Money::new(667, 50)),
        (25, 25, None, Money::new(898, 75)),
        (35, 35, None, Money::new(1396, 25)),
        (50, 50, None, Money::new(2202, 50)),
        (63, 63, None, Money::new(2948, 75)),
    ]))
    .transfer_fee(TransferFee::Simple(
        Cost::fuse_range_with_yearly_consumption(&[
            (16, 16, Some(8000), Money::new_subunit(63.80)),
            (16, 63, None, Money::new_subunit(25.00)),
        ]),
    ))
    .build();

pub const NORD: GridOperator = BASE
    .name("E.ON Nord")
    .price_date(2024, 1, 1)
    .monthly_fee(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(216, 25)),
        (16, 16, None, Money::new(697, 50)),
        (20, 20, None, Money::new(892, 50)),
        (25, 25, None, Money::new(1131, 25)),
        (35, 35, None, Money::new(1615, 00)),
        (50, 50, None, Money::new(2355, 00)),
        (63, 63, None, Money::new(3011, 25)),
    ]))
    .transfer_fee(TransferFee::Simple(
        Cost::fuse_range_with_yearly_consumption(&[
            (16, 16, Some(8000), Money::new_subunit(98.50)),
            (16, 63, None, Money::new_subunit(26.30)),
        ]),
    ))
    .build();
