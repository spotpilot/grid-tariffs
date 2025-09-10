use crate::registry::prelude::*;
pub const SOLLENTUNA_ENERGI_MILJO_AB: GridOperator = GridOperator::builder()
    .name("Sollentuna Energi & Miljö AB")
    .vat_number("SE559457224901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder("https://www.seom.se/el/elnat/priser-och-villkor/")
            .plain_content_locator(".main-content-area")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(137, 50)),
            (25, Money::new(137, 50)),
            (35, Money::new(245, 0)),
            (50, Money::new(345, 0)),
            (63, Money::new(420, 0)),
            (80, Money::new(536, 67)),
            (100, Money::new(665, 0)),
            (125, Money::new(816, 67)),
            (160, Money::new(1050, 0)),
            (200, Money::new(1330, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unlisted)
        .other_fees(OtherFees::List(&[("Energiavgift", Money::new_subunit(5.))]))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[
                // NOTE: "Under helger och röda dagar tas ingen effektavgift ut"
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fixed(135, 0))
                    .months(November, March)
                    .hours(7, 19)
                    .exclude_weekends_and_swedish_holidays()
                    .build(),
                CostPeriod::builder()
                    .load(Low)
                    .cost(Cost::fixed(67, 50))
                    .months(April, October)
                    .hours(7, 19)
                    .exclude_weekends_and_swedish_holidays()
                    .build(),
            ]),
        ))
        .build()])
    .build();
