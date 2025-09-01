use crate::registry::prelude::*;

pub const AFFÄRSVERKEN_ELNÄT_AB: GridOperator = GridOperator::builder()
    .name("Affärsverken Elnät AB")
    .vat_number("SE556532083401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .price_date(2025, 1, 1)
    .monthly_fee(Cost::fuses(&[
        (16, Money::new(345, 0)),
        (20, Money::new(476, 0)),
        (25, Money::new(643, 0)),
        (35, Money::new(1009, 0)),
        (50, Money::new(1520, 0)),
        (63, Money::new(2010, 0)),
    ]))
    .monthly_production_fee(Cost::Unverified)
    .feed_in_revenue(FeedInRevenue::Unverified)
    .transfer_fee(TransferFee::SpotPriceVariable {
        base_cost: Cost::fixed_subunit(31.60),
        spot_price_multiplier: 0.077,
        approximated: false,
    })
    .other_fees(OtherFees::Unverified)
    .links(Links::new(
        Link::builder("https://www.affarsverken.se/elnat/elnatspriser/privatkund/")
            .plain_content_locator("#content-body")
            .build(),
    ))
    .power_tariff(PowerTariff::NotImplemented)
    .build();
