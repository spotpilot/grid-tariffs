use crate::registry::prelude::*;

const FEE_LINK: &str = "https://borgholmenergi.se/privat/elnat/priser-och-avtal/natpriser/";

pub const BORGHOLM_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Borgholm Energi Elnät AB")
    .vat_number("SE556020762201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 80))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4678, 0).divide_by(12)),
            (20, Money::new(6976, 0).divide_by(12)),
            (25, Money::new(9023, 0).divide_by(12)),
            (35, Money::new(13067, 0).divide_by(12)),
            (50, Money::new(19318, 0).divide_by(12)),
            (63, Money::new(25190, 0).divide_by(12)),
            (80, Money::new(37261, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(26.1))
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
