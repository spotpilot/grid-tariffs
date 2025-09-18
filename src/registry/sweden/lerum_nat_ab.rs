use crate::registry::prelude::*;

pub static LERUM_NAT_AB: GridOperator = GridOperator::builder()
    .name("Lerum Nät AB")
    .vat_number("SE556109395501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 640))
    .links(
        Links::builder()
            .new_fee_info(
                "https://www.lerumenergi.se/Elnat/Elnatsavgifter",
                "#MainContent",
            )
            .new_feed_in_revenue_info(
                "https://www.lerumenergi.se/Solenergi/Salj-ditt-overskott",
                "#MainContent",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2730, 0).divide_by(12)),
            (20, Money::new(3563, 0).divide_by(12)),
            (25, Money::new(4406, 0).divide_by(12)),
            (35, Money::new(6611, 0).divide_by(12)),
            (50, Money::new(9053, 0).divide_by(12)),
            (63, Money::new(13548, 0).divide_by(12)),
            (80, Money::new(14561, 0).divide_by(12)),
            (100, Money::new(18545, 0).divide_by(12)),
            (125, Money::new(19766, 0).divide_by(12)),
            (160, Money::new(20975, 0).divide_by(12)),
            (200, Money::new(25883, 0).divide_by(12)),
            (225, Money::new(37708, 0).divide_by(12)),
            (250, Money::new(45056, 0).divide_by(12)),
            (315, Money::new(49709, 0).divide_by(12)),
            (320, Money::new(54434, 0).divide_by(12)),
            (400, Money::new(69120, 0).divide_by(12)),
            (500, Money::new(76463, 0).divide_by(12)),
            (600, Money::new(83805, 0).divide_by(12)),
            (640, Money::new(91963, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(38.95))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
