use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.alemenergi.se/elnat/natavgifter/";

pub static ALEM_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ålem Energi Elnät AB")
    .vat_number("SE556179650801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(458, 0)),
            (20, Money::new(779, 0)),
            (25, Money::new(1012, 0)),
            (35, Money::new(1507, 0)),
            (50, Money::new(2285, 0)),
            (63, Money::new(2993, 0)),
            (80, Money::new(3721, 0)),
            (100, Money::new(4764, 0)),
            (125, Money::new(6116, 0)),
            (160, Money::new(8310, 0)),
            (200, Money::new(11179, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(27.65))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
