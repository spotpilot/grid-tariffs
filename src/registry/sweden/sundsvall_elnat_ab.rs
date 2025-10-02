use crate::registry::prelude::*;

// TODO: This link seems to only be relevant for 2025 pricing. Add a concept for warning about expiring links!
const FEE_LINK: &str = "https://sundsvallelnat.se/min-anslutning/priser-och-avtalsvillkor/2025-sakringsabonnemang-privat";

pub static SUNDSVALL_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Sundsvall Elnät AB")
    .vat_number("SE556502722301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(4943, 0).divide_by(12)),
            (20, Money::new(7670, 0).divide_by(12)),
            (25, Money::new(10408, 0).divide_by(12)),
            (35, Money::new(14340, 0).divide_by(12)),
            (50, Money::new(20068, 0).divide_by(12)),
            (63, Money::new(25851, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(25.1))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
