use crate::registry::prelude::*;

const FEE_LINK: &str = "https://nackansenergi.se/elnat/natavgifter/";

pub static NACKANS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Näckåns Elnät AB")
    .vat_number("SE556526699501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("div.page")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3620, 0).divide_by(12)),
            (20, Money::new(4800, 0).divide_by(12)),
            (25, Money::new(5945, 0).divide_by(12)),
            (35, Money::new(8560, 0).divide_by(12)),
            (50, Money::new(12440, 0).divide_by(12)),
            (63, Money::new(15920, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(28.75))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
