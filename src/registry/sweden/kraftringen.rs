use crate::registry::prelude::*;

pub const KRAFTRINGEN: GridOperator = GridOperator::builder()
    .name("Kraftringen")
    .vat_number("SE556228113801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(
            "https://www.kraftringen.se/privat/elnat/elnatsavgifter/komplett-elnatsprislista/",
        )
        .plain_content_locator(".main-page-content")
        .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 6, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(685, 0)),
            (20, Money::new(1080, 0)),
            (25, Money::new(1345, 0)),
            (35, Money::new(1765, 0)),
            (50, Money::new(2470, 0)),
            (63, Money::new(3075, 0)),
            (80, Money::new(4115, 0)),
            (100, Money::new(5205, 0)),
            (125, Money::new(6850, 0)),
            (160, Money::new(8850, 0)),
            (200, Money::new(11030, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .transfer_fee(TransferFee::SpotPriceVariable {
            base_cost: Money::new_subunit(20.0),
            spot_price_multiplier: 0.05,
            approximated: false,
        })
        .feed_in_revenue(FeedInRevenue::Unlisted)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
