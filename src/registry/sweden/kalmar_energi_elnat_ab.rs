use crate::registry::prelude::*;

pub static KALMAR_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Kalmar Energi Elnät AB")
    .vat_number("SE556182754301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info("https://kalmarenergi.se/el/elnat/elnatsavgiften/", "main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4375, 0).divide_by(12)),
            (20, Money::new(7092, 0).divide_by(12)),
            (25, Money::new(8936, 0).divide_by(12)),
            (35, Money::new(13291, 0).divide_by(12)),
            (50, Money::new(19228, 0).divide_by(12)),
            (63, Money::new(21424, 0).divide_by(12)),
            (80, Money::new(22749, 0).divide_by(12)),
            (100, Money::new(24860, 0).divide_by(12)),
            (125, Money::new(31484, 0).divide_by(12)),
            (160, Money::new(41881, 0).divide_by(12)),
            (200, Money::new(53263, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        // Calculated from examples
        // TODO: Verify
        .feed_in_revenue(FeedInRevenue::spot_price_variable(5.62, 0.07, true))
        // Calculated from examples
        // TODO: Verify
        .transfer_fee(TransferFee::spot_price_variable(5.0, 0.0648, true))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
