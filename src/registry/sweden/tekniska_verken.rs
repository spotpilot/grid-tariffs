use crate::registry::prelude::*;

const WINTER_MONTHS: PeriodType = PeriodType::Months(Months::new(November, March));
const SUMMER_MONTHS: PeriodType = PeriodType::Months(Months::new(April, October));
const NIGHT_HOURS: PeriodType = PeriodType::Hours(Hours::new(6, 22));
const DAY_HOURS: PeriodType = PeriodType::Hours(Hours::new(23, 5));

const LINKS: Links = Links {
    fee_info: "https://www.tekniskaverken.se/kundservice/priser-avtal/priser-elnat-2025/",
    eltariff_api: Some("https://api.tekniskaverken.net/subscription/public/v0/tariffs"),
};

pub const KATRINEHOLM_STANDARD: GridOperator = GridOperator {
    name: "Tekniska Verken Katrineholm, prislista standard",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    // NOTE: "Myndighetsavgift" @ 131,82 kr/year is not included
    monthly_fee: Cost::fuses(&[
        (16, Money::new(325, 0)),
        (20, Money::new(415, 0)),
        (25, Money::new(560, 0)),
        (35, Money::new(750, 0)),
        (50, Money::new(995, 0)),
        (63, Money::new(1275, 0)),
    ]),
    transfer_fee: TransferFee::fixed_subunit(17.40),
    monthly_production_fee: Cost::None,
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.40),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDays(5),
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(63, 0)
                .include(WINTER_MONTHS)
                .build(),
            CostPeriod::builder().fixed_cost(32, 0).build(),
        ]),
    )),
    links: LINKS,
};

pub const KATRINEHOLM_ALTERNATIV: GridOperator = GridOperator {
    name: "Tekniska Verken Katrineholm, prislista alternativ",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    // NOTE: "Myndighetsavgift" @ 131,82 kr/year is not included
    monthly_fee: Cost::fuses(&[
        (16, Money::new(325, 0)),
        (20, Money::new(415, 0)),
        (25, Money::new(560, 0)),
        (35, Money::new(750, 0)),
        (50, Money::new(995, 0)),
        (63, Money::new(1275, 0)),
    ]),
    transfer_fee: TransferFee::TimeOfDay {
        day: Cost::fixed_subunit(21.70),
        night: Cost::fixed_subunit(10.80),
    },
    monthly_production_fee: Cost::None,
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.40),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDayNightDifferentiated { day: 1, night: 1 },
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(11, 0)
                .include(SUMMER_MONTHS)
                .include(DAY_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(32, 0)
                .include(SUMMER_MONTHS)
                .include(NIGHT_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(16, 0)
                .include(WINTER_MONTHS)
                .include(DAY_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(63, 0)
                .include(WINTER_MONTHS)
                .include(NIGHT_HOURS)
                .build(),
        ]),
    )),
    links: LINKS,
};

pub const LINKÖPING_STANDARD: GridOperator = GridOperator {
    name: "Tekniska Verken Linköping, prislista standard",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    // NOTE: "Myndighetsavgift" @ 131,82 kr/year is not included
    monthly_fee: Cost::fuses(&[
        (16, Money::new(160, 0)),
        (20, Money::new(220, 0)),
        (25, Money::new(300, 0)),
        (35, Money::new(480, 0)),
        (50, Money::new(655, 0)),
        (63, Money::new(805, 0)),
    ]),
    transfer_fee: TransferFee::fixed_subunit(14.30),
    monthly_production_fee: Cost::None,
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.40),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDays(5),
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(43, 0)
                .include(WINTER_MONTHS)
                .build(),
            CostPeriod::builder().fixed_cost(22, 0).build(),
        ]),
    )),
    links: LINKS,
};

pub const LINKÖPING_ALTERNATIV: GridOperator = GridOperator {
    name: "Tekniska Verken Linköping, prislista alternativ",
    price_date: date(2025, 1, 1),
    country: Country::SE,
    main_fuses: MainFuseSizes::new_range(16, 63),
    // NOTE: "Myndighetsavgift" @ 131,82 kr/year is not included
    monthly_fee: Cost::fuses(&[
        (16, Money::new(160, 0)),
        (20, Money::new(220, 0)),
        (25, Money::new(300, 0)),
        (35, Money::new(480, 0)),
        (50, Money::new(655, 0)),
        (63, Money::new(805, 0)),
    ]),
    transfer_fee: TransferFee::TimeOfDay {
        day: Cost::fixed_subunit(17.30),
        night: Cost::fixed_subunit(8.70),
    },
    monthly_production_fee: Cost::None,
    feed_in_revenue: FeedInRevenue::fixed_subunit(5.40),
    other_fees: OtherFees::Unverified,
    power_tariff: Some(PowerTariff::new(
        TariffCalculationMethod::AverageDayNightDifferentiated { day: 1, night: 1 },
        CostPeriods::new(&[
            CostPeriod::builder()
                .fixed_cost(8, 0)
                .include(SUMMER_MONTHS)
                .include(DAY_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(22, 0)
                .include(SUMMER_MONTHS)
                .include(NIGHT_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(12, 0)
                .include(WINTER_MONTHS)
                .include(DAY_HOURS)
                .build(),
            CostPeriod::builder()
                .fixed_cost(43, 0)
                .include(WINTER_MONTHS)
                .include(NIGHT_HOURS)
                .build(),
        ]),
    )),
    links: LINKS,
};
