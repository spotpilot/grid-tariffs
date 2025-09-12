use crate::registry::prelude::*;

const BASE_PRICELIST: PriceListBuilder = PriceListBuilder::new()
    .from_date(2025, 9, 1)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::new_periods(CostPeriods::new(&[
        CostPeriod::builder()
            .load(High)
            .fixed_cost_subunit(9.11)
            .months(November, March)
            .hours(6, 22)
            .exclude_weekends()
            .build(),
        CostPeriod::builder()
            .load(Low)
            .fixed_cost_subunit(4.91)
            .build(),
    ])));

pub const MALARENERGI: GridOperator = GridOperator::builder()
    .name("Mälarenergi")
    .vat_number("SE556554150401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.malarenergi.se/el/elnat/priser-elnat/")
            .plain_content_locator(".standard-article")
            .build(),
    ))
    .price_lists(&[
        BASE_PRICELIST
            .variant("Säkringsabonnemang")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(307, 50)),
                (20, Money::new(335, 0)),
                (25, Money::new(362, 50)),
                (35, Money::new(437, 50)),
                (50, Money::new(540, 0)),
                (63, Money::new(625, 0)),
            ]))
            .transfer_fee(TransferFee::fixed_subunit(36.25))
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::PeakHour,
                CostPeriods::new(&[CostPeriod::builder()
                    .load(High)
                    .fixed_cost(18, 75)
                    .hours(7, 19)
                    .exclude_weekends_and_swedish_holidays()
                    .build()]),
            ))
            .build(),
        BASE_PRICELIST
            .variant("Effektabonnemang, lågspänning")
            .monthly_fee(Cost::fixed(896, 00))
            .transfer_fee(TransferFee::fixed(0, 09))
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::PeakHour,
                CostPeriods::new(&[CostPeriod::builder()
                    .load(High)
                    .cost(Cost::fixed(69, 68).add_vat(Country::SE))
                    .hours(7, 19)
                    .exclude_weekends_and_swedish_holidays()
                    .build()]),
            ))
            .build(),
    ])
    .build();
