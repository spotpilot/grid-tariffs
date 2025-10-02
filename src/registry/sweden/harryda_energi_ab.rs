use crate::registry::prelude::*;

pub static HARRYDA_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Härryda Energi AB")
    .vat_number("SE556026324501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info("https://harrydaenergi.se/elnat/elnatspriser/", "#content")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2024, 4, 1)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::fuses(&[
            (16, Money::new(4650, 0).divide_by(12)),
            (20, Money::new(5800, 0).divide_by(12)),
            (25, Money::new(7220, 0).divide_by(12)),
            (35, Money::new(9900, 0).divide_by(12)),
            (50, Money::new(14150, 0).divide_by(12)),
            (63, Money::new(17770, 0).divide_by(12)),
            (80, Money::new(22570, 0).divide_by(12)),
            (100, Money::new(27920, 0).divide_by(12)),
            (125, Money::new(34660, 0).divide_by(12)),
            (160, Money::new(44070, 0).divide_by(12)),
            (200, Money::new(54790, 0).divide_by(12)),
        ]))
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(23.9))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
