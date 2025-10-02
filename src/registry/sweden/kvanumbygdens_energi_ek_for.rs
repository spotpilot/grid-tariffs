use crate::registry::prelude::*;

const FEE_LINK: &str = "https://kvanumenergi.se/elnat/natavgift/";

pub static KVANUMBYGDENS_ENERGI_EK_FOR: GridOperator = GridOperator::builder()
    .name("Kvänumbygdens Energi ek. för")
    .vat_number("SE768400219401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#text_block-6-11020")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4500, 0).divide_by(12)),
            (20, Money::new(5688, 0).divide_by(12)),
            (25, Money::new(7200, 0).divide_by(12)),
            (35, Money::new(10008, 0).divide_by(12)),
            (50, Money::new(14544, 0).divide_by(12)),
            (63, Money::new(18216, 0).divide_by(12)),
            (80, Money::new(23076, 0).divide_by(12)),
            (100, Money::new(28963, 0).divide_by(12)),
            (125, Money::new(36216, 0).divide_by(12)),
            (160, Money::new(45738, 0).divide_by(12)),
            (200, Money::new(59670, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(21.6))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
