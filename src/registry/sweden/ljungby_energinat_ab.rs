use crate::registry::prelude::*;

const FEE_LINK: &str = "https://ljungby-energi.se/elnat/elnatsavgifter/";

pub static LJUNGBY_ENERGINAT_AB: GridOperator = GridOperator::builder()
    .name("Ljungby Energinät AB")
    .vat_number("SE556082399801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3174, 0).divide_by(12)),
            (20, Money::new(4564, 0).divide_by(12)),
            (25, Money::new(5690, 0).divide_by(12)),
            (35, Money::new(9425, 0).divide_by(12)),
            (50, Money::new(14286, 0).divide_by(12)),
            (63, Money::new(20538, 0).divide_by(12)),
            (80, Money::new(28643, 0).divide_by(12)),
            (100, Money::new(39523, 0).divide_by(12)),
            (125, Money::new(55423, 0).divide_by(12)),
            (160, Money::new(74714, 0).divide_by(12)),
            (200, Money::new(104808, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::spot_price_variable(8.5, 0.09, false))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
