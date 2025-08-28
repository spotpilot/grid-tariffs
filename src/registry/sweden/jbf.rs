use crate::registry::prelude::*;

pub const JBF: GridOperator = GridOperator {
    name: "Jukkasjärvi Sockens Belysningsförening",
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 1500),
    price_date: date(2025, 1, 1),
    monthly_fee: Cost::fuse_range(&[
        (16, 63, Money::new(560, 17)),
        (64, 1500, Money::new(2431, 0)),
    ]),
    monthly_production_fee: Cost::Unverified,
    feed_in_revenue: FeedInRevenue::Unverified,
    transfer_fee: TransferFee::Unlisted,
    other_fees: OtherFees::Unverified,
    links: Links {
        eltariff_api: None,
        fee_info: "https://jbf.nu/elnatet/natavgift/",
    },
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageHours(3),
        CostPeriods::new(&[
            CostPeriod::builder()
                .cost(Cost::fuse_range(&[
                    (16, 63, Money::new(44, 0)),
                    (80, 1500, Money::new(48, 0)),
                ]))
                .fallthrough(true)
                .build(),
            CostPeriod::builder()
                .cost(Cost::fuse_range(&[
                    (16, 25, Money::new(106, 0)),
                    (35, 63, Money::new(210, 0)),
                    (80, 1500, Money::new(226, 0)),
                ]))
                .include_months(November, March)
                .include_hours(6, 22)
                .exclude_weekends_and_swedish_holidays()
                .build(),
        ]),
    )),
};
