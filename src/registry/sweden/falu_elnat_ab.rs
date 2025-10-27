use crate::registry::prelude::*;

pub static FALU_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Falu Elnät AB")
    .vat_number("SE556002847301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://fev.se/el/elnat/elnatspriser-privat.html",
                ".pagecontent",
            )
            .feed_in_revenue_info(
                "https://fev.se/el/elnat/anslut-till-elnatet/producera-egen-el.html",
                ".pagecontent",
            )
            .build(),
    )
    .price_lists(&[
        PriceList::builder()
            .from_date(2024, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(400, 0)),
                (20, Money::new(520, 0)),
                (25, Money::new(650, 0)),
                (35, Money::new(890, 0)),
                (50, Money::new(1235, 0)),
                (63, Money::new(1565, 0)),
            ]))
            .monthly_production_fee(Cost::None)
            .feed_in_revenue(FeedInRevenue::fixed_subunit(3.54))
            .transfer_fee(TransferFee::fixed_subunit(15.0))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            .from_date(2025, 11, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(395, 0)),
                (20, Money::new(480, 0)),
                (25, Money::new(585, 0)),
                (35, Money::new(800, 0)),
                (50, Money::new(1125, 0)),
                (63, Money::new(1400, 0)),
            ]))
            .monthly_production_fee(Cost::None)
            .feed_in_revenue(FeedInRevenue::fixed_subunit(3.54))
            .transfer_fee(TransferFee::fixed_subunit(11.25))
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::AverageDays(3),
                CostPeriods::new_first(&[
                    CostPeriod::builder()
                        .load(High)
                        .months(November, March)
                        .hours(7, 19)
                        .exclude_weekends()
                        .exclude_holidays(Country::SE)
                        .fixed_cost(75, 0)
                        .build(),
                    CostPeriodBuilder::new().load(Low).fixed_cost(0, 0).build(),
                ]),
            ))
            .build(),
    ])
    .build();
