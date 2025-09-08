use crate::registry::prelude::*;

pub const VARBERG_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Varberg Energi Elnät AB")
    .vat_number("SE556013145901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder(
            "https://www.varbergenergi.se/privat/tjanster/varberg-energi-elnat-ab/elnatspriser/",
        )
        .plain_content_locator(".accordion-list > :nth-child(-n + 2)")
        .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(9999, 12, 31)
        .monthly_fee(Cost::Unverified)
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::Unverified)
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
