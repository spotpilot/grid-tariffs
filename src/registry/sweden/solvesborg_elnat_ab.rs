use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.solvesborgenergi.se/elnat/avgifter-for-elnat";

pub static SOLVESBORG_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Sölvesborg Elnät AB")
    .vat_number("SE559497572301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4400, 0).divide_by(12)),
            (20, Money::new(6560, 0).divide_by(12)),
            (25, Money::new(9245, 0).divide_by(12)),
            (35, Money::new(13480, 0).divide_by(12)),
            (50, Money::new(21340, 0).divide_by(12)),
            (63, Money::new(29410, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(25.00))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
