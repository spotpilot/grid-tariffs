use crate::registry::prelude::*;

pub const SUNDSVALL_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Sundsvall Elnät AB")
    .vat_number("SE556502722301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(Link::builder("https://sundsvallelnat.se/min-anslutning/priser-och-avtalsvillkor/2025-sakringsabonnemang-privat").plain_content_locator("main").build()))
    // .power_tariff(PowerTariff::new(
    //     TariffCalculationMethod::PeakHour,
    //     CostPeriods::new(&[
    //         CostPeriod::builder().load(Low).build(),
    //         CostPeriod::builder().load(High).build(),
    //     ]),
    // ))
    .build();
