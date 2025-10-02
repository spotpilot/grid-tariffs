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
            TariffCalculationMethod::PeakHour,
            CostPeriods::new(&[
                CostPeriod::builder().load(Base).fixed_cost(56, 25).build(),
                CostPeriod::builder()
                    .load(High)
                    .months(November, March)
                    .hours(7, 18)
                    .fixed_cost(43, 75)
                    .exclude_weekends_and_swedish_holidays()
                    .build(),
            ]),
        ))
        .build()])
    .build();
