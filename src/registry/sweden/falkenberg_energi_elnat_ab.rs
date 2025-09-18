use crate::registry::prelude::*;

pub static FALKENBERG_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Falkenberg Energi Elnät AB")
    .vat_number("SE556461483101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.falkenberg-energi.se/elnat/elnatspriser/sakringsabonnemang/")
            .plain_content_locator(".entry-content")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5100, 0).divide_by(12)),
            (20, Money::new(7800, 0).divide_by(12)),
            (25, Money::new(9825, 0).divide_by(12)),
            (35, Money::new(14475, 0).divide_by(12)),
            (50, Money::new(19950, 0).divide_by(12)),
            (63, Money::new(24900, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::SpotPriceVariable {
            base_cost: Money::new_subunit(9.16),
            spot_price_multiplier: 5.11,
            approximated: false,
        })
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
