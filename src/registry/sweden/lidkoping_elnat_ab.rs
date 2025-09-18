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
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3415, 0).divide_by(12)),
            (20, Money::new(5036, 0).divide_by(12)),
            (25, Money::new(6321, 0).divide_by(12)),
            (35, Money::new(9720, 0).divide_by(12)),
            (50, Money::new(13794, 0).divide_by(12)),
            (63, Money::new(18350, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(14.1))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
