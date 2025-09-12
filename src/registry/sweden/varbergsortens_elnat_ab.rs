use crate::registry::prelude::*;

const FEE_LINK: &str = "https://vbgelkraft.se/elnat/elnatpriser/";

pub const VARBERGSORTENS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Varbergsortens Elnät AB")
    .vat_number("SE559481776801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(20, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#content")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 4, 1)
        .monthly_fee(Cost::fuses(&[
            (20, Money::new(5063, 0).divide_by(12)),
            (25, Money::new(7163, 0).divide_by(12)),
            (35, Money::new(10625, 0).divide_by(12)),
            (50, Money::new(16188, 0).divide_by(12)),
            (63, Money::new(20625, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(20.0))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
