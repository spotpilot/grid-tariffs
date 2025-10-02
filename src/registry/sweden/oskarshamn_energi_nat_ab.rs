use crate::registry::prelude::*;

pub static OSKARSHAMN_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Oskarshamn Energi Nät AB")
    .vat_number("SE556275876201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info(
                "https://www.oskarshamnenergi.se/privat/elnat/natavgifter",
                "main",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5258, 0).divide_by(12)),
            (20, Money::new(7895, 0).divide_by(12)),
            (25, Money::new(9267, 0).divide_by(12)),
            (35, Money::new(16983, 0).divide_by(12)),
            (50, Money::new(20272, 0).divide_by(12)),
            (63, Money::new(25253, 0).divide_by(12)),
            (80, Money::new(31762, 0).divide_by(12)),
            (100, Money::new(39422, 0).divide_by(12)),
            (125, Money::new(48991, 0).divide_by(12)),
            (160, Money::new(62396, 0).divide_by(12)),
            (200, Money::new(77712, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::None)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(7.5))
        // TODO: https://github.com/spotpilot/grid-tariffs/issues/152
        .transfer_fee(TransferFee::spot_price_variable(9.705, 0.0561, false))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
