use crate::registry::prelude::*;

const FEE_LINK: &str = "https://ssel.se/elnat/elnatsavgifter/";

pub static SANDHULTSANDAREDS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("SandhultSandareds Elnät AB")
    .vat_number("SE559461745701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 600))
    .links(Links::builder().fee_info(FEE_LINK, "#main-content").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3621, 0).divide_by(12)),
            (20, Money::new(5452, 0).divide_by(12)),
            (25, Money::new(7043, 0).divide_by(12)),
            (35, Money::new(11149, 0).divide_by(12)),
            (50, Money::new(17001, 0).divide_by(12)),
            (63, Money::new(21427, 0).divide_by(12)),
            (80, Money::new(27890, 0).divide_by(12)),
            (100, Money::new(34002, 0).divide_by(12)),
            (125, Money::new(42503, 0).divide_by(12)),
            (160, Money::new(55092, 0).divide_by(12)),
            (200, Money::new(72086, 0).divide_by(12)),
            (250, Money::new(89109, 0).divide_by(12)),
            (315, Money::new(110511, 0).divide_by(12)),
            (400, Money::new(136031, 0).divide_by(12)),
            (500, Money::new(170039, 0).divide_by(12)),
            (600, Money::new(204048, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(24.1))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
