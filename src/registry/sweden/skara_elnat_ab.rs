use crate::registry::prelude::*;

const FEE_LINK: &str = "https://skaraenergi.se/genvagar/priser-och-avtalsvillkor/";

pub static SKARA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Skara Elnät AB")
    .vat_number("SE559441591001")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2634, 0).divide_by(12)),
            (20, Money::new(4367, 0).divide_by(12)),
            (25, Money::new(5459, 0).divide_by(12)),
            (35, Money::new(7632, 0).divide_by(12)),
            (50, Money::new(10907, 0).divide_by(12)),
            (63, Money::new(13891, 0).divide_by(12)),
            (80, Money::new(18130, 0).divide_by(12)),
            (100, Money::new(22037, 0).divide_by(12)),
            (125, Money::new(27560, 0).divide_by(12)),
            (160, Money::new(35277, 0).divide_by(12)),
            (200, Money::new(43979, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(22.50))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
