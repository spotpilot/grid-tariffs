use crate::registry::prelude::*;

pub static JBF: GridOperator = GridOperator::builder()
    .name("Jukkasjärvi Sockens Belysningsförening")
    .vat_number("SE797300037001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 1500))
    .links(
        Links::builder()
            .fee_info("https://jbf.nu/elnatet/natavgift/", "#main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuse_range(&[
            (16, 63, Money::new(560, 17)),
            (64, 1500, Money::new(2431, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unlisted)
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fuse_range(&[
                        (16, 25, Money::new(44 + 106, 0)),
                        (35, 63, Money::new(44 + 210, 0)),
                        (80, 1500, Money::new(48 + 226, 0)),
                    ]))
                    .months(November, March)
                    .hours(6, 22)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .cost(Cost::fuse_range(&[
                        (16, 63, Money::new(44, 0)),
                        (80, 1500, Money::new(48, 0)),
                    ]))
                    .build(),
            ]),
        ))
        .build()])
    .build();
