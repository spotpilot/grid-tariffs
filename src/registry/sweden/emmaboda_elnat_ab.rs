use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.emmabodaenergi.se/elnat/vad-kostar-det.html";

pub static EMMABODA_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Emmaboda Elnät AB")
    .vat_number("SE556459927101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 250))
    .links(
        Links::builder()
            .fee_info_default(FEE_LINK)
            .feed_in_revenue_info_default(
                "https://www.emmabodaenergi.se/elnat/mikroproducent/ersattning-for-natnytta.html",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4125, 0).divide_by(12)),
            (20, Money::new(5250, 0).divide_by(12)),
            (25, Money::new(7625, 0).divide_by(12)),
            (35, Money::new(12500, 0).divide_by(12)),
            (50, Money::new(18750, 0).divide_by(12)),
            (63, Money::new(25750, 0).divide_by(12)),
            (80, Money::new(30625, 0).divide_by(12)),
            (100, Money::new(45250, 0).divide_by(12)),
            (125, Money::new(57500, 0).divide_by(12)),
            (160, Money::new(75500, 0).divide_by(12)),
            (200, Money::new(102000, 0).divide_by(12)),
            (250, Money::new(108375, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(6.5))
        .transfer_fee(TransferFee::fixed_subunit(32.87))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
