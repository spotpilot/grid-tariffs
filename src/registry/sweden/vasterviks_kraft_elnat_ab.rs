use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.vmeab.se/tjanster/elnat/priser--villkor/";

pub const VASTERVIKS_KRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Västerviks Kraft Elnät AB")
    .vat_number("SE556528138201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("main")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4140, 0).divide_by(12)),
            (20, Money::new(5400, 0).divide_by(12)),
            (25, Money::new(7140, 0).divide_by(12)),
            (35, Money::new(10310, 0).divide_by(12)),
            (50, Money::new(14780, 0).divide_by(12)),
            (63, Money::new(19530, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(8.25))
        .transfer_fee(TransferFee::fixed_subunit(31.40))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
