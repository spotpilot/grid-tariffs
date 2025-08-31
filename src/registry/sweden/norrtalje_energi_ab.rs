use crate::registry::prelude::*;

pub const NORRTÄLJE_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Norrtälje Energi AB")
    .vat_number("SE556399224601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(9999, 12, 31)
    .monthly_fee(Cost::Unverified)
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::Unverified)
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://www.norrtaljeenergi.se/elnat/elnatspriser-elpriser/")
            .plain_content_locator(".price-slider")
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
