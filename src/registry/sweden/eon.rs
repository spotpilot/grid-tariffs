use crate::registry::prelude::*;

const LINKS: Links = Links {
    fee_info: "https://www.eon.se/nyheter/ett-effektivare-elnaet",
    eltariff_api: None,
};

pub(super) const SYD: GridOperator = GridOperator {
    name: "E.ON Syd",
    price_date: date(2024, 8, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(215, 0)),
        (16, 16, None, Money::new(591, 25)),
        (20, 20, None, Money::new(780, 0)),
        (25, 25, None, Money::new(1006, 25)),
        (35, 35, None, Money::new(1472, 50)),
        (50, 50, None, Money::new(2203, 75)),
        (63, 63, None, Money::new(2865, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    // 92,5% correlation between spot price and grid benefit from my own finding
    feed_in_revenue: FeedInRevenue::SpotPriceVariable {
        base_cost: Cost::fixed_subunit(4.58),
        spot_price_multiplier: 0.06,
        approximated: true,
    },
    transfer_fee: TransferFee::Simple(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new_subunit(87.15)),
        (16, 63, None, Money::new_subunit(30.70)),
    ])),
    other_fees: OtherFees::Unverified,
    power_tariff: None,
    links: LINKS,
};

pub(super) const STOCKHOLM: GridOperator = GridOperator {
    name: "E.ON Stockholm",
    price_date: date(2025, 7, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(215, 00)),
        (16, 16, None, Money::new(473, 75)),
        (20, 20, None, Money::new(667, 50)),
        (25, 25, None, Money::new(898, 75)),
        (35, 35, None, Money::new(1396, 25)),
        (50, 50, None, Money::new(2202, 50)),
        (63, 63, None, Money::new(2948, 75)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Simple(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new_subunit(63.80)),
        (16, 63, None, Money::new_subunit(25.00)),
    ])),
    other_fees: OtherFees::Unverified,
    power_tariff: None,
    links: LINKS,
};

pub(super) const NORD: GridOperator = GridOperator {
    name: "E.ON Nord",
    price_date: date(2024, 1, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new(216, 25)),
        (16, 16, None, Money::new(697, 50)),
        (20, 20, None, Money::new(892, 50)),
        (25, 25, None, Money::new(1131, 25)),
        (35, 35, None, Money::new(1615, 00)),
        (50, 50, None, Money::new(2355, 00)),
        (63, 63, None, Money::new(3011, 25)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Simple(Cost::fuse_range_with_yearly_consumption(&[
        (16, 16, Some(8000), Money::new_subunit(98.50)),
        (16, 63, None, Money::new_subunit(26.30)),
    ])),
    other_fees: OtherFees::Unverified,
    power_tariff: None,
    links: LINKS,
};
