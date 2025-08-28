use crate::{builder::GridOperatorBuilder, registry::prelude::*};

const BASE: GridOperatorBuilder = GridOperator::builder()
    .price_date(2025,1,1)
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .monthly_fee(Cost::fuses(&[
        (16, Money::new(420, 83)),
        (20, Money::new(588, 75)),
        (25, Money::new(737, 50)),
        (35, Money::new(1011, 67)),
        (50, Money::new(1452, 50)),
        (63, Money::new(1957, 92)),
    ]))
    .links(Links::new("https://www.vattenfalleldistribution.se/abonnemang-och-avgifter/avtal-och-avgifter/elnatsavgift-och-avtalsvillkor/"));

pub const VATTENFALL_E4: GridOperator = BASE
    .name("Vattenfall E4")
    .transfer_fee(TransferFee::fixed_subunit(39.0))
    .build();

pub const VATTENFALL_T4: GridOperator = BASE
    .name("Vattenfall T4")
    .transfer_fee(TransferFee::new_periods(CostPeriods::new(&[
        CostPeriod::builder()
            .fixed_cost_subunit(67.00)
            .include_months(November, March)
            .include_hours(6, 22)
            .exclude_weekends()
            .build(),
        CostPeriod::builder().fixed_cost_subunit(26.80).build(),
    ])))
    .build();
