use crate::registry::prelude::*;

pub static LANDSKRONA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Landskrona Energi AB")
    .vat_number("SE556803921701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info(
                "https://landskronaenergi.se/privat/el/elnatsavgifter/",
                ".elementor-price-list",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3969, 0).divide_by(12)),
            (20, Money::new(5995, 0).divide_by(12)),
            (25, Money::new(8403, 0).divide_by(12)),
            (35, Money::new(12895, 0).divide_by(12)),
            (50, Money::new(18819, 0).divide_by(12)),
            (63, Money::new(24363, 0).divide_by(12)),
            (80, Money::new(36268, 0).divide_by(12)),
            (100, Money::new(46363, 0).divide_by(12)),
            (125, Money::new(57303, 0).divide_by(12)),
            (160, Money::new(74313, 0).divide_by(12)),
            (200, Money::new(89355, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(24.463))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
