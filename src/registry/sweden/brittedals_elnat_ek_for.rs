use crate::registry::prelude::*;

const FEE_LINK: &str = "https://brittedal.se/elnatspriser";

pub const BRITTEDALS_ELNAT_EK_FOR: GridOperator = GridOperator::builder()
    .name("Brittedals Elnät ek för")
    .vat_number("SE737000010601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(580, 0)),
            (20, Money::new(810, 0)),
            (25, Money::new(1054, 0)),
            (35, Money::new(1578, 0)),
            (50, Money::new(2333, 0)),
            (63, Money::new(2878, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::SpotPriceVariable {
            base_cost: Cost::fixed_subunit(14.0),
            spot_price_multiplier: 0.09,
            approximated: false,
        })
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
