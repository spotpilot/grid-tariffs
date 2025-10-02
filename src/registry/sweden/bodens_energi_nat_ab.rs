use crate::registry::prelude::*;

pub static BODENS_ENERGI_NAT_AB: GridOperator = GridOperator::builder()
    .name("Bodens Energi Nät AB")
    .vat_number("SE556526856101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info("https://bodensenergi.se/elnat/elnatspriser/", "main")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3810, 0).divide_by(12)),
            (20, Money::new(9100, 0).divide_by(12)),
            (25, Money::new(11060, 0).divide_by(12)),
            (35, Money::new(15110, 0).divide_by(12)),
            (50, Money::new(21100, 0).divide_by(12)),
            (63, Money::new(26500, 0).divide_by(12)),
            (80, Money::new(33875, 0).divide_by(12)),
            (100, Money::new(42145, 0).divide_by(12)),
            (125, Money::new(52150, 0).divide_by(12)),
            (160, Money::new(66310, 0).divide_by(12)),
            (200, Money::new(82445, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Simple(Cost::Fuses(&[(
            16,
            Money::new_subunit(22.25),
        )])))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
