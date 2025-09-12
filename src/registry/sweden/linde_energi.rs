use crate::registry::prelude::*;

pub const LINDE_ENERGI: GridOperator = GridOperator::builder()
    .name("Linde Energi")
    .vat_number("SE556468527801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.lindeenergi.se/elnat/elnatspriser")
            .plain_content_locator("#Innehall + div")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuse_range(&[
            (16, 16, Money::new(384, 42)),
            (20, 20, Money::new(482, 33)),
            (25, 25, Money::new(597, 92)),
            (35, 63, Money::new(505, 21)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fuse_range(&[
            (16, 25, Money::new_subunit(30.5)),
            (35, 63, Money::new_subunit(6.125)),
        ])))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDaysDifferentiated { base: 2, peak: 2 },
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(Base)
                    .cost(Cost::fuse_range(&[
                        (16, 25, Money::ZERO),
                        (35, 63, Money::new(75, 0)),
                    ]))
                    .build(),
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fuse_range(&[
                        (16, 25, Money::ZERO),
                        (35, 63, Money::new(46, 75)),
                    ]))
                    .months(November, March)
                    .hours(7, 19)
                    .exclude_weekends()
                    .build(),
            ]),
        ))
        .build()])
    .build();
