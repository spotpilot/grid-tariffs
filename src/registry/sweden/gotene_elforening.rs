use crate::registry::prelude::*;

const FEE_LINK: &str = "https://kinnekulleenergi.se/avgifter-villkor/";

pub static GOTENE_ELFORENING: GridOperator = GridOperator::builder()
    .name("Götene Elförening")
    .vat_number("SE769000061201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://kinnekulleenergi.se/mikroproduktion/")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4778, 0).divide_by(12)),
            (20, Money::new(8094, 0).divide_by(12)),
            (25, Money::new(9748, 0).divide_by(12)),
            (35, Money::new(14238, 0).divide_by(12)),
            (50, Money::new(21174, 0).divide_by(12)),
            (63, Money::new(29420, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(2.4))
        .transfer_fee(TransferFee::fixed_subunit(23.13))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
