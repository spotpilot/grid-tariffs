use crate::registry::prelude::*;

pub const VARBERG_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Varberg Energi Elnät AB")
    .vat_number("SE556013145901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::new(
        Link::builder(
            "https://www.varbergenergi.se/privat/tjanster/varberg-energi-elnat-ab/elnatspriser/",
        )
        .plain_content_locator(".accordion-list > :nth-child(-n + 2)")
        .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 9, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(2428, 00).add_vat(Country::SE).divide_by(12)),
            (20, Money::new(3380, 00).add_vat(Country::SE).divide_by(12)),
            (25, Money::new(5988, 00).add_vat(Country::SE).divide_by(12)),
            (35, Money::new(9760, 00).add_vat(Country::SE).divide_by(12)),
            (50, Money::new(13048, 00).add_vat(Country::SE).divide_by(12)),
            (63, Money::new(17540, 00).add_vat(Country::SE).divide_by(12)),
            (80, Money::new(22936, 00).add_vat(Country::SE).divide_by(12)),
            (
                100,
                Money::new(28544, 00).add_vat(Country::SE).divide_by(12),
            ),
            (
                125,
                Money::new(34924, 00).add_vat(Country::SE).divide_by(12),
            ),
            (
                160,
                Money::new(45840, 00).add_vat(Country::SE).divide_by(12),
            ),
            (
                200,
                Money::new(55692, 00).add_vat(Country::SE).divide_by(12),
            ),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(9.5)) //OBS räknat pris inkl moms manuellt (7,6*1,25)
        .other_fees(OtherFees::Unverified)
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
