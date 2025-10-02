use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.levailysekil.se/elnat/investeringsresan/taxor-elnat.html";

pub static LEVA_NAT_AB: GridOperator = GridOperator::builder()
    .name("LEVA Nät AB")
    .vat_number("SE556508344001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4971, 0).divide_by(12)),
            (20, Money::new(8165, 0).divide_by(12)),
            (25, Money::new(10418, 0).divide_by(12)),
            (35, Money::new(15220, 0).divide_by(12)),
            (50, Money::new(22274, 0).divide_by(12)),
            (63, Money::new(28748, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(5.625))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
