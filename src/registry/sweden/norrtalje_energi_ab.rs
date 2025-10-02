use crate::registry::prelude::*;

pub static NORRTALJE_ENERGI_AB: GridOperator = GridOperator::builder()
    .name("Norrtälje Energi AB")
    .vat_number("SE556399224601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.norrtaljeenergi.se/elnat/elnatspriser-elpriser/",
                ".price-slider",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 4, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4716, 0).divide_by(12)),
            (20, Money::new(6983, 0).divide_by(12)),
            (25, Money::new(8893, 0).divide_by(12)),
            (35, Money::new(12805, 0).divide_by(12)),
            (50, Money::new(19796, 0).divide_by(12)),
            (63, Money::new(27978, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(27.0))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
