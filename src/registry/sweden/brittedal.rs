use crate::registry::prelude::*;

pub(super) const BRITTEDAL: GridOperator = GridOperator {
    name: "Brittedal",
    price_date: date(2025, 1, 1),
    currency: Currency::SEK,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::Unverified,
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Simple(Cost::Unverified),
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://brittedal.se/elnatspriser",
    },
    power_tariff: None,
};