use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.teknikivast.se/sidor/elnat/ditt-elnatsabonnemang/din-natavgift/natavgift-sakringskund.html";

pub const ARVIKA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Arvika Elnät AB")
    .vat_number("SE556527671301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator(".pagecontent")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4050, 0).divide_by(12)),
            (20, Money::new(6188, 0).divide_by(12)),
            (25, Money::new(7631, 0).divide_by(12)),
            (35, Money::new(10000, 0).divide_by(12)),
            (50, Money::new(13013, 0).divide_by(12)),
            (63, Money::new(19875, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(24.9))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
