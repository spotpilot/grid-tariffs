use crate::registry::prelude::*;

pub const ESKILSTUNA_ENERGI_MILJÖ: GridOperator = GridOperator {
    name: "Eskilstuna Energi & Miljö",
    vat_number: "SE556513955601",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    // NOTE: They have a power tariff based price model implemented for 35A+, but we'll skip adding those for now...
    main_fuses: MainFuseSizes::new_range(16, 25),
    monthly_fee: Cost::fuses(&[
        (16, Money::new(361, 67)),
        (20, Money::new(475, 42)),
        (25, Money::new(577, 50)),
    ]),
    transfer_fee: TransferFee::fixed_subunit(29.40),
    monthly_production_fee: Cost::fixed(25, 0),
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.0),
    other_fees: OtherFees::Unverified,
    power_tariff: None,
    links: Links::new(
        Link::builder("https://www.eem.se/privat/elnat/priser-och-avgifter/elnatsavgift-2025")
            .plain_content_locator("article")
            .build(),
    ),
};
