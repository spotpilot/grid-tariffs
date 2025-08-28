use crate::registry::prelude::*;

pub const GISLAVED_ENERGI: GridOperator = GridOperator {
    name: "Gislaved Energi",
    price_date: date(2024, 12, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(291, 58)),
        (20, Money::new(397, 25)),
        (25, Money::new(467, 25)),
        (35, Money::new(658, 33)),
        (50, Money::new(969, 42)),
        (63, Money::new(1246, 58)),
    ]),
    monthly_production_fee: Cost::Unverified,
    // TODO: They call it "rörlig avgift", not "överföringsavgift"..?
    transfer_fee: TransferFee::fixed_subunit(30.61),
    // TODO: They don't list if it's with or without taxes
    feed_in_revenue: FeedInRevenue::fixed_subunit(4.2),
    other_fees: OtherFees::Unverified,
    power_tariff: None,
    links: Links::new(
        "https://www.gislavedenergi.se/el/elnat/elnatsavgifter/elnatsavgifter-sakringsabonnemang/",
    ),
};
