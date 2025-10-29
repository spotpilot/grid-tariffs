use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://www.luleaenergi.se/produktion-och-infrastruktur/elnat/natpriser-och-avtalsvillkor";

pub static LULEA_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Luleå Energi Elnät AB")
    .vat_number("SE556527753901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(
        Links::builder()
            .fee_info(FEE_LINK, "main section:first-of-type")
            .feed_in_revenue_info_default("https://www.luleaenergi.se/produktion-och-infrastruktur/elnat/natpriser-och-avtalsvillkor")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3250, 0).divide_by(12)),
            (20, Money::new(7490, 0).divide_by(12)),
            (25, Money::new(8820, 0).divide_by(12)),
            (35, Money::new(11920, 0).divide_by(12)),
            (50, Money::new(16040, 0).divide_by(12)),
            (63, Money::new(19940, 0).divide_by(12)),
            (80, Money::new(30600, 0).divide_by(12)),
            (100, Money::new(37030, 0).divide_by(12)),
            (125, Money::new(45490, 0).divide_by(12)),
            (160, Money::new(61580, 0).divide_by(12)),
            (200, Money::new(78200, 0).divide_by(12)),
            (250, Money::new(101820, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(2.0))
        .transfer_fee(TransferFee::Simple(Cost::fuse_range(&[
            (16, 16, Money::new_subunit(17.5)),
            (20, 250, Money::ZERO),
        ])))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
