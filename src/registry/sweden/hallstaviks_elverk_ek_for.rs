use crate::registry::prelude::*;

const FEE_LINK: &str = "https://hallstavikselverk.se/elnat/elnatspriser/";

pub static HALLSTAVIKS_ELVERK_EK_FOR: GridOperator = GridOperator::builder()
    .name("Hallstaviks Elverk ek för")
    .vat_number("SE714400051501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(
        Links::builder()
            .fee_info(FEE_LINK, ".gital-page-container")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3145, 0).divide_by(12)),
            (20, Money::new(3949, 0).divide_by(12)),
            (25, Money::new(4919, 0).divide_by(12)),
            (35, Money::new(6906, 0).divide_by(12)),
            (50, Money::new(9841, 0).divide_by(12)),
            (63, Money::new(12439, 0).divide_by(12)),
            (80, Money::new(15795, 0).divide_by(12)),
            (100, Money::new(19701, 0).divide_by(12)),
            (125, Money::new(24622, 0).divide_by(12)),
            (160, Money::new(28221, 0).divide_by(12)),
            (200, Money::new(35307, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(32.04))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
