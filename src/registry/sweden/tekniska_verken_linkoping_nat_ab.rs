use crate::registry::prelude::*;

pub const TEKNISKA_VERKEN_LINKÖPING_NÄT_AB: GridOperator = GridOperator::builder()
    .name("Tekniska verken Linköping Nät AB")
    .vat_number("SE556483492601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(Link::builder("https://www.tekniskaverken.se/kundservice/priser-avtal/priser-elnat-2025/#konsumtionsabonnemang").plain_content_locator("main").build()))
    // .power_tariff(PowerTariff::new(
    //     TariffCalculationMethod::PeakHour,
    //     CostPeriods::new(&[
    //         CostPeriod::builder().load(Low).build(),
    //         CostPeriod::builder().load(High).build(),
    //     ]),
    // ))
    .build();
