use crate::registry::prelude::*;

pub const LULEÅ_ENERGI_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Luleå Energi Elnät AB")
    .vat_number("SE556527753901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(Link::builder("https://www.luleaenergi.se/produktion-och-infrastruktur/elnat/natpriser-och-avtalsvillkor?referer=1087").content_locator_default().build()))
    // .power_tariff(PowerTariff::new(
    //     TariffCalculationMethod::PeakHour,
    //     CostPeriods::new(&[
    //         CostPeriod::builder().load(Low).build(),
    //         CostPeriod::builder().load(High).build(),
    //     ]),
    // ))
    .build();
