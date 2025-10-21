use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.alingsasenergi.se/elnatsavgift/";

pub static ALINGSAS_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Alingsås Energi Elnät AB")
    .vat_number("SE559434718801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(25, 63))
    .links(
        Links::builder()
            .fee_info_default("https://www.alingsasenergi.se/elnatsavgift/ ")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (25, Money::new(4598, 0).divide_by(12)),
            (35, Money::new(10354, 0).divide_by(12)),
            (50, Money::new(16500, 0).divide_by(12)),
            (63, Money::new(24049, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(4.50))
        .transfer_fee(TransferFee::fixed_subunit(23.80))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
