use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.gavleenergi.se/elnat/elnatspriser/";

pub const GAVLE_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Gävle Energi Elnät AB")
    .vat_number("SE559397122801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 50))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4150, 0).divide_by(12)),
            (20, Money::new(6145, 0).divide_by(12)),
            (25, Money::new(8260, 0).divide_by(12)),
            (35, Money::new(10200, 0).divide_by(12)),
            (50, Money::new(15090, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(15.0))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
