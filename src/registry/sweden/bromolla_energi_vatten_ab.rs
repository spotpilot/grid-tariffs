use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.bromolla.se/bevab/om-elmarknaden-och-elnat/elpriser/";

pub static BROMOLLA_ENERGI_VATTEN_AB: GridOperator = GridOperator::builder()
    .name("Bromölla Energi & Vatten AB")
    .vat_number("SE556525863801")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_LINK)
            .plain_content_locator("#wrapper")
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
