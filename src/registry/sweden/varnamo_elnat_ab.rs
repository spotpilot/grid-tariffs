use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.varnamoenergi.se/abonnemangsavgifter-elnat";

pub static VARNAMO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Värnamo Elnät AB")
    .vat_number("SE556528056601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 35))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default("https://www.varnamoenergi.se/natnyttoersattning")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2772, 50).divide_by(12)),
            (20, Money::new(4012, 50).divide_by(12)),
            (25, Money::new(5087, 50).divide_by(12)),
            (35, Money::new(8262, 50).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(10.0))
        .transfer_fee(TransferFee::fixed_subunit(34.20))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
