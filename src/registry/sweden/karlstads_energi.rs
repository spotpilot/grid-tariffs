use crate::registry::prelude::*;

pub const KARLSTADS_ENERGI: GridOperator = GridOperator {
    name: "Karlstads Energi",
    vat_number: "SE556527673901",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, u16::MAX),
    monthly_fee: Cost::fuse_range(&[
        (16, 63, Money::new(145, 83)),
        (63, u16::MAX, Money::new(642, 19)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Simple(Cost::fixed_subunit(9.75)),
    other_fees: OtherFees::Unverified,
    links: Links::new(
        Link::builder("https://karlstadsenergi.se/gemensamt-innehall/elnat/priser/elnatsavgifter")
            .plain_content_locator("table")
            .build(),
    ),
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::PeakHour,
        // NOTE: The pricing is actually the same for 16-63A as >63A if you account for VAT vs without VAT
        CostPeriods::new(&[
            CostPeriod::builder().load(Base).fixed_cost(43, 55).build(),
            CostPeriod::builder()
                .load(High)
                .fixed_cost(74, 15)
                .hours(6, 18)
                .exclude_weekends_and_swedish_holidays()
                .build(),
        ]),
    )),
};
