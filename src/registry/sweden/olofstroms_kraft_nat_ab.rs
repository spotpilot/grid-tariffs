use crate::registry::prelude::*;

pub static OLOFSTROMS_KRAFT_NAT_AB: GridOperator = GridOperator::builder()
    .name("Olofströms Kraft Nät AB")
    .vat_number("SE556462112501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.olofstromskraft.se/natavgifter-el",
                ".main-content",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2024, 11, 14)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4732, 0).divide_by(12)),
            (20, Money::new(6806, 0).divide_by(12)),
            (25, Money::new(8617, 0).divide_by(12)),
            (35, Money::new(12547, 0).divide_by(12)),
            (50, Money::new(15562, 0).divide_by(12)),
            (63, Money::new(21825, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(24.73))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
