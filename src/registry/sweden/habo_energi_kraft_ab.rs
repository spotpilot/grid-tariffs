use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.haboenergi.se/natavgiften-privat/";

pub static HABO_ENERGI_KRAFT_AB: GridOperator = GridOperator::builder()
    .name("Habo Energi Kraft AB")
    .vat_number("SE556058980501")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("[data-elementor-type='wp-page']")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4390, 0).divide_by(12)),
            (20, Money::new(6553, 0).divide_by(12)),
            (25, Money::new(9009, 0).divide_by(12)),
            (35, Money::new(12333, 0).divide_by(12)),
            (50, Money::new(17585, 0).divide_by(12)),
            (63, Money::new(22959, 0).divide_by(12)),
            (80, Money::new(30819, 0).divide_by(12)),
            (100, Money::new(38513, 0).divide_by(12)),
            (125, Money::new(48420, 0).divide_by(12)),
            (160, Money::new(62106, 0).divide_by(12)),
            (200, Money::new(77463, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(22.89))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
