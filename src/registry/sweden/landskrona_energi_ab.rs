use crate::registry::prelude::*;

pub const LANDSKRONA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Landskrona Energi AB")
    .vat_number("SE556803921701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://landskronaenergi.se/privat/el/elnatsavgifter/")
            .plain_content_locator("main")
            .build(),
    ))
    // .power_tariff(PowerTariff::new(
    //     TariffCalculationMethod::PeakHour,
    //     CostPeriods::new(&[
    //         CostPeriod::builder().load(Low).build(),
    //         CostPeriod::builder().load(High).build(),
    //     ]),
    // ))
    .build();
