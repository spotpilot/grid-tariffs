use crate::registry::prelude::*;

pub static BORLANGE_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Borlänge Energi Elnät AB")
    .vat_number("SE556478968201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.borlange-energi.se/elnat/vad-kostar-elnatet",
                "main",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2100, 0).divide_by(12)),
            (20, Money::new(2550, 0).divide_by(12)),
            (25, Money::new(3100, 0).divide_by(12)),
            (35, Money::new(4025, 0).divide_by(12)),
            (50, Money::new(4775, 0).divide_by(12)),
            (63, Money::new(6050, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::fuses(&[
            (16, Money::new_subunit(32.0)),
            (20, Money::new_subunit(31.0)),
            (25, Money::new_subunit(29.5)),
            (35, Money::new_subunit(29.5)),
            (50, Money::new_subunit(29.5)),
            (63, Money::new_subunit(29.5)),
        ])))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
