use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.vemab.se/tjanster/el/elnat/natavgifter";

pub static VIMMERBY_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Vimmerby Energi Nät AB")
    .vat_number("SE559011498801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[
        PriceList::builder()
            .variant("Normaltariff")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(3685, 0).divide_by(12)),
                (20, Money::new(7250, 0).divide_by(12)),
                (25, Money::new(8921, 0).divide_by(12)),
                (35, Money::new(11750, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::fixed_subunit(42.8))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
        PriceList::builder()
            .variant("Mellantariff")
            .from_date(2025, 1, 1)
            .monthly_fee(Cost::fuses(&[
                (16, Money::new(6600, 0).divide_by(12)),
                (20, Money::new(9285, 0).divide_by(12)),
                (25, Money::new(11952, 0).divide_by(12)),
                (35, Money::new(16627, 0).divide_by(12)),
                (50, Money::new(23106, 0).divide_by(12)),
                (63, Money::new(30135, 0).divide_by(12)),
                (80, Money::new(37513, 0).divide_by(12)),
                (100, Money::new(47526, 0).divide_by(12)),
                (125, Money::new(59125, 0).divide_by(12)),
                (160, Money::new(76527, 0).divide_by(12)),
                (200, Money::new(92576, 0).divide_by(12)),
            ]))
            .monthly_production_fee(Cost::Unverified)
            .feed_in_revenue(FeedInRevenue::Unverified)
            .transfer_fee(TransferFee::fixed_subunit(25.9))
            .power_tariff(PowerTariff::NotImplemented)
            .build(),
    ])
    .build();
