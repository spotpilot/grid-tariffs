use crate::registry::prelude::*;

pub const VASTERBERGSLAGENS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Västerbergslagens Elnät AB")
    .vat_number("SE556565686401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.vbenergi.se/elnat/elnatsavtalet2/elnatspriser--avtalsvillkor/")
            .plain_content_locator("#page article")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4905, 0).divide_by(12)),
            (20, Money::new(6900, 0).divide_by(12)),
            (25, Money::new(8540, 0).divide_by(12)),
            (35, Money::new(11850, 0).divide_by(12)),
            (50, Money::new(17135, 0).divide_by(12)),
            (63, Money::new(23065, 0).divide_by(12)),
        ])
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(40.0))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
