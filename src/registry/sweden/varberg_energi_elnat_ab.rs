use crate::registry::prelude::*;

pub static VARBERG_ENERGI_ELNAT_AB: GridOperator = GridOperator::builder()
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
            (16, Money::new(3035, 00).divide_by(12)),
            (20, Money::new(4225, 00).divide_by(12)),
            (25, Money::new(7485, 00).divide_by(12)),
            (35, Money::new(12200, 00).divide_by(12)),
            (50, Money::new(16310, 00).divide_by(12)),
            (63, Money::new(21925, 00).divide_by(12)),
            (80, Money::new(28670, 00).divide_by(12)),
            (100, Money::new(35680, 00).divide_by(12)),
            (125, Money::new(43655, 00).divide_by(12)),
            (160, Money::new(57300, 00).divide_by(12)),
            (200, Money::new(69615, 00).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(9.5))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageHours(3),
            CostPeriods::new(&[CostPeriod::builder()
                .load(Base)
                .cost(Cost::fixed(30, 00))
                .build()]),
        ))
        .build()])
    .build();
