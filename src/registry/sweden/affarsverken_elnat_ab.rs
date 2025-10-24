use crate::registry::prelude::*;

pub static AFFARSVERKEN_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Affärsverken Elnät AB")
    .vat_number("SE556532083401")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(
                "https://www.affarsverken.se/elnat/elnatspriser/privatkund/",
                "#content-body",
            )
            .feed_in_revenue_info_default(
                "https://www.affarsverken.se/elnat/for-elproducenter/ersattning-for-elproduktion/",
            )
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(345, 0)),
            (20, Money::new(476, 0)),
            (25, Money::new(643, 0)),
            (35, Money::new(1009, 0)),
            (50, Money::new(1520, 0)),
            (63, Money::new(2010, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(7.5))
        .transfer_fee(TransferFee::spot_price_variable(31.60, 0.077, false))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
