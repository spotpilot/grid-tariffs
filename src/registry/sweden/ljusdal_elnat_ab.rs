use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.ljusdalenergi.se/elnat/natpriser.html";

pub static LJUSDAL_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ljusdal Elnät AB")
    .vat_number("SE556509989101")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info(FEE_LINK, ".pagecontent").build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3650, 0).divide_by(12)),
            (20, Money::new(5255, 0).divide_by(12)),
            (25, Money::new(7280, 0).divide_by(12)),
            (35, Money::new(10155, 0).divide_by(12)),
            (50, Money::new(14550, 0).divide_by(12)),
            (63, Money::new(19905, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(17.50))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
