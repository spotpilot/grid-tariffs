use crate::registry::prelude::*;

pub static HOGANAS_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Höganäs Energi Elnät AB")
    .vat_number("SE556440240101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder("https://www.hoganasenergi.se/elnat/prislistor")
            .plain_content_locator(".pagecontent")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4125, 0).divide_by(12)),
            (20, Money::new(6720, 0).divide_by(12)),
            (25, Money::new(8655, 0).divide_by(12)),
            (35, Money::new(12155, 0).divide_by(12)),
            (50, Money::new(17940, 0).divide_by(12)),
            (63, Money::new(25065, 0).divide_by(12)),
            (80, Money::new(32595, 0).divide_by(12)),
            (100, Money::new(40720, 0).divide_by(12)),
            (125, Money::new(51940, 0).divide_by(12)),
            (160, Money::new(75065, 0).divide_by(12)),
            (200, Money::new(99375, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        // "** Det rörliga volympriset är beroende av elpriset på elbörsen Nordpool och överliggande näts debitering till Höganäs Energi. Priset fastställs månadsvis och framgår av respektive månads faktura."
        // TODO: Find out actual cost
        .transfer_fee(TransferFee::spot_price_variable(21.00, 5.0, true))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
