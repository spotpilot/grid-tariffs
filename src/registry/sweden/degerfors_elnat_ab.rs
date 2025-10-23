use crate::registry::prelude::*;

const FEE_LINK: &str = "https://degerforsenergi.se/elnat/effekttaxa-galler-fran-1-9-2024/";

pub static DEGERFORS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Degerfors Elnät AB")
    .vat_number("SE559440444301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2024, 9, 1)
        .monthly_fee(Cost::fixed(3000, 0))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(17.5))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(1),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(7, 18)
                    // base + high = 56,25 + 43,75
                    .fixed_cost(100, 0)
                    .exclude_weekends()
                    .exclude_holidays(Country::SE)
                    .build(),
                CostPeriod::builder().load(Low).fixed_cost(56, 25).build(),
            ]),
        ))
        .build()])
    .build();
