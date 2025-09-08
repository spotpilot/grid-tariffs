use crate::registry::prelude::*;

const SUMMER: CostPeriodBuilder = CostPeriodBuilder::new().months(April, October);
const WINTER: CostPeriodBuilder = CostPeriodBuilder::new().months(November, March);

const NIGHT_HOURS: PeriodType = PeriodType::Hours(Hours::new(6, 22));
const DAY_HOURS: PeriodType = PeriodType::Hours(Hours::new(23, 5));

const SUMMER_DAYS: CostPeriodBuilder = SUMMER.include(DAY_HOURS);
const SUMMER_NIGHTS: CostPeriodBuilder = SUMMER.include(NIGHT_HOURS);

const WINTER_DAYS: CostPeriodBuilder = WINTER.include(DAY_HOURS);
const WINTER_NIGHTS: CostPeriodBuilder = WINTER.include(NIGHT_HOURS);

const BASE_PRICELIST_2025: PriceListBuilder = PriceListBuilder::new()
    .from_date(2025, 1, 1)
    .feed_in_revenue(FeedInRevenue::fixed_subunit(5.40))
    .monthly_production_fee(Cost::Unverified)
    .other_fees(OtherFees::Unverified);

const BASE: GridOperatorBuilder = GridOperatorBuilder::new()
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                Link::builder(
                    "https://www.tekniskaverken.se/kundservice/priser-avtal/priser-elnat-2025/",
                )
                .plain_content_locator("main")
                .build(),
            )
            .eltariff_api("https://api.tekniskaverken.net/subscription/public/v0/tariffs")
            .build(),
    );

pub const TEKNISKA_VERKEN_LINKOPING: GridOperator = BASE
    .name("Tekniska Verken Linköping")
    .vat_number("SE556483492601")
    .price_lists(&[
        BASE_PRICELIST_2025
            .variant("Prislista standard")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(160, 0)),
                (20, Money::new(220, 0)),
                (25, Money::new(300, 0)),
                (35, Money::new(480, 0)),
                (50, Money::new(655, 0)),
                (63, Money::new(805, 0)),
            ]))
            .transfer_fee(TransferFee::fixed_subunit(14.30))
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::AverageDays(5),
                CostPeriods::new(&[
                    SUMMER.load(Low).fixed_cost(22, 0).build(),
                    WINTER.load(High).fixed_cost(43, 0).build(),
                ]),
            ))
            .build(),
        BASE_PRICELIST_2025
            .variant("Prislista alternativ")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(160, 0)),
                (20, Money::new(220, 0)),
                (25, Money::new(300, 0)),
                (35, Money::new(480, 0)),
                (50, Money::new(655, 0)),
                (63, Money::new(805, 0)),
            ]))
            .transfer_fee(TransferFee::TimeOfDay {
                day: Cost::fixed_subunit(17.30),
                night: Cost::fixed_subunit(8.70),
            })
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::AverageDayNightDifferentiated { day: 1, night: 1 },
                CostPeriods::new(&[
                    SUMMER_NIGHTS.load(Low).fixed_cost(8, 0).build(),
                    SUMMER_DAYS.load(High).fixed_cost(22, 0).build(),
                    WINTER_NIGHTS.load(Low).fixed_cost(12, 0).build(),
                    WINTER_DAYS.load(High).fixed_cost(43, 0).build(),
                ]),
            ))
            .build(),
    ])
    .build();

pub const TEKNISKA_VERKEN_KATRINEHOLM: GridOperator = BASE
    .name("Tekniska Verken Katrineholm")
    .vat_number("SE556426858801")
    .price_lists(&[
        BASE_PRICELIST_2025
            .variant("Prislista standard")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(325, 0)),
                (20, Money::new(415, 0)),
                (25, Money::new(560, 0)),
                (35, Money::new(750, 0)),
                (50, Money::new(995, 0)),
                (63, Money::new(1275, 0)),
            ]))
            .transfer_fee(TransferFee::fixed_subunit(17.40))
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::AverageDays(5),
                CostPeriods::new(&[
                    SUMMER.load(Low).fixed_cost(32, 0).build(),
                    WINTER.load(High).fixed_cost(63, 0).build(),
                ]),
            ))
            .build(),
        BASE_PRICELIST_2025
            .variant("Prislista alternativ")
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(325, 0)),
                (20, Money::new(415, 0)),
                (25, Money::new(560, 0)),
                (35, Money::new(750, 0)),
                (50, Money::new(995, 0)),
                (63, Money::new(1275, 0)),
            ]))
            .transfer_fee(TransferFee::TimeOfDay {
                day: Cost::fixed_subunit(21.70),
                night: Cost::fixed_subunit(10.80),
            })
            .power_tariff(PowerTariff::new(
                TariffCalculationMethod::AverageDayNightDifferentiated { day: 1, night: 1 },
                CostPeriods::new(&[
                    SUMMER_NIGHTS.load(Low).fixed_cost(11, 0).build(),
                    SUMMER_DAYS.load(High).fixed_cost(32, 0).build(),
                    WINTER_NIGHTS.load(Low).fixed_cost(16, 0).build(),
                    WINTER_DAYS.load(High).fixed_cost(63, 0).build(),
                ]),
            ))
            .build(),
    ])
    .build();
