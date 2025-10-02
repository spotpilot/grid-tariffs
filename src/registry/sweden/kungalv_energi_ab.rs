use crate::registry::prelude::*;

pub static KUNGALV_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Kungälv Energi AB")
    .vat_number("SE556083206401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.kungalvenergi.se/elnat/priser-for-elnat-2025/",
                ".mainbody",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::fuses(&[
            (16, Money::new(4104, 0).divide_by(12)),
            (20, Money::new(4729, 0).divide_by(12)),
            (25, Money::new(6011, 0).divide_by(12)),
            (35, Money::new(8154, 0).divide_by(12)),
            (50, Money::new(11521, 0).divide_by(12)),
            (63, Money::new(14459, 0).divide_by(12)),
        ]))
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(24.625))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
