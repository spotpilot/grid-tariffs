use crate::registry::prelude::*;

const FEE_URL: &str = "https://lidkoping.se/lidkoping-miljo-och-teknik-ab/start/elnat-och-elforsorjning/abonnemang-avgifter-och-avtal";

pub static LIDKOPING_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Lidköping Elnät AB")
    .vat_number("SE559395998301")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(FEE_URL)
            .plain_content_locator("#h-Elnatsavgifter + *")
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
