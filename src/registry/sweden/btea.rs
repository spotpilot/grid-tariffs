use crate::registry::prelude::*;

pub static BTEA: GridOperator = GridOperator::builder()
    .name("BTEA")
    .vat_number("SE556012264901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.btea.se/elnat/elnatspriser")
            .plain_content_locator("table")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 10, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5754, 0).divide_by(12)),
            (20, Money::new(6919, 0).divide_by(12)),
            (25, Money::new(7986, 0).divide_by(12)),
            (35, Money::new(13074, 0).divide_by(12)),
            (50, Money::new(18340, 0).divide_by(12)),
            (63, Money::new(22720, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fixed_subunit(2.50)))
        .power_tariff(PowerTariff::new(
            // TODO: We need to differentiate between high load and low load hours..., Not day and night...
            TariffCalculationMethod::PeakHours(&[High, Low]),
            #[cfg_attr(rustfmt, rustfmt_skip)]
        CostPeriods::new(&[
            // Very strange that they charge 0 kr for the high load periods...
            LOW_LOAD.months(October, November).fixed_cost(51, 25).build(),
            LOW_LOAD_HOURS.months(December, January).fixed_cost(56, 25).build(),
            HIGH_LOAD_HOURS.months(December, January).fixed_cost(158, 75).build(),
            LOW_LOAD_HOURS.month(February).fixed_cost(53, 75).build(),
            HIGH_LOAD_HOURS.month(February).fixed_cost(137, 50).build(),
            LOW_LOAD.months(March, May).fixed_cost(51, 25).build(),
            LOW_LOAD.months(June, September).fixed_cost(37, 50).build(),
        ]),
        ))
        .build()])
    .build();

const LOW_LOAD: CostPeriodBuilder = CostPeriod::builder().load(Low);
const LOW_LOAD_HOURS: CostPeriodBuilder = LOW_LOAD.hours(22, 5);
const HIGH_LOAD_HOURS: CostPeriodBuilder = CostPeriod::builder().load(High).hours(6, 21);
