use crate::registry::prelude::*;

pub const JAMTKRAFT_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Jämtkraft Elnät AB")
    .vat_number("SE556103399301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 125))
    .links(Links::new(
        Link::builder("https://www.jamtkraft.se/privat/elnat/elnatsavgifter/")
            .plain_content_locator("article")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5440, 0).divide_by(12)),
            (20, Money::new(8900, 0).divide_by(12)),
            (25, Money::new(11320, 0).divide_by(12)),
            (35, Money::new(16130, 0).divide_by(12)),
            (50, Money::new(23130, 0).divide_by(12)),
            (63, Money::new(29880, 0).divide_by(12)),
            (80, Money::new(40250, 0).divide_by(12)),
            (100, Money::new(50750, 0).divide_by(12)),
            (125, Money::new(63880, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::fixed_yearly(240, 0))
        .feed_in_revenue(FeedInRevenue::fixed_subunit(4.0))
        .transfer_fee(TransferFee::fixed_subunit(7.50))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
