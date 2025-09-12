use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.gavleenergi.se/elnat/elnatspriser/";

pub const GAVLE_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Gävle Energi Elnät AB")
    .vat_number("SE559397122801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 50))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[
        PriceList::builder()
            .variant("Standard")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(4150, 0).divide_by(12)),
                (20, Money::new(6145, 0).divide_by(12)),
                (25, Money::new(8260, 0).divide_by(12)),
                (35, Money::new(10200, 0).divide_by(12)),
                (50, Money::new(15090, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::fixed_subunit(8.0))
            .transfer_fee(TransferFee::fixed_subunit(15.0))
            .other_fees(OtherFees::Unverified)
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            // TODO: Because of company merge, this is only active during 2025. Delete by 2026-01-01 (and handle migration in some way).
            .variant("Hofors")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(4415, 0).divide_by(12)),
                (20, Money::new(7696, 0).divide_by(12)),
                (25, Money::new(9874, 0).divide_by(12)),
                (35, Money::new(14168, 0).divide_by(12)),
                (50, Money::new(20760, 0).divide_by(12)),
                (63, Money::new(26799, 0).divide_by(12)),
                (80, Money::new(34960, 0).divide_by(12)),
                (100, Money::new(44873, 0).divide_by(12)),
                (125, Money::new(57546, 0).divide_by(12)),
                (160, Money::new(80445, 0).divide_by(12)),
                (200, Money::new(96764, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::fixed_subunit(8.0))
            .transfer_fee(TransferFee::Simple(Cost::FuseRange(&[
                (16, 16, Money::new_subunit(24.74)),
                (20, 200, Money::new_subunit(12.00)),
            ])))
            .other_fees(OtherFees::Unverified)
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
    ])
    .build();
