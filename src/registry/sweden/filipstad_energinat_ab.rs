use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.filipstadenerginat.se/forkunder/avgifterochvillkor.64.html";

pub static FILIPSTAD_ENERGINAT_AB: GridOperator = GridOperator::builder()
    .name("Filipstad Energinät AB")
    .vat_number("SE556517499101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(388, 0)),
            (20, Money::new(490, 0)),
            (25, Money::new(645, 0)),
            (35, Money::new(895, 0)),
            (50, Money::new(1365, 0)),
            (63, Money::new(1730, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(30.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
