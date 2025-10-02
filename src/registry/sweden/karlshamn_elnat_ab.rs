use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.karlshamnenergi.se/el/elnat/priser/";

pub static KARLSHAMN_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Karlshamn Elnät AB")
    .vat_number("SE559440734701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2075, 0).divide_by(12)),
            (20, Money::new(2688, 0).divide_by(12)),
            (25, Money::new(3494, 0).divide_by(12)),
            (35, Money::new(4856, 0).divide_by(12)),
            (50, Money::new(6756, 0).divide_by(12)),
            (63, Money::new(8825, 0).divide_by(12)),
            (80, Money::new(11425, 0).divide_by(12)),
            (100, Money::new(14931, 0).divide_by(12)),
            (125, Money::new(19406, 0).divide_by(12)),
            (160, Money::new(27250, 0).divide_by(12)),
            (200, Money::new(35519, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::SpotPriceVariable {
            base_cost: Money::ZERO,
            spot_price_multiplier: 0.0890,
            approximated: false,
        })
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
