use crate::registry::prelude::*;

pub const ELLEVIO: GridOperator = GridOperator::builder()
    .name("Ellevio")
    .vat_number("SE556037732601")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::new(
        Link::builder("https://www.ellevio.se/abonnemang/elnatspriser-privat/")
            .content_locator(ContentLocator::new_starts_with(
                "För bland annat villor, radhus, fritidshus och verksamhetslokaler med egen",
                TargetContainer::Parent,
                ContentTarget::TextWithLinks,
            ))
            .build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(365, 0)),
            (20, Money::new(365, 0)),
            (25, Money::new(365, 0)),
            (35, Money::new(915, 0)),
            (50, Money::new(1400, 0)),
            (63, Money::new(2010, 0)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(6.25))
        .power_tariff(PowerTariff::new(
            TariffCalculationMethod::AverageDays(3),
            CostPeriods::new(&[
                CostPeriod::builder()
                    .load(Base)
                    .cost(Cost::fixed(81, 25))
                    .build(),
                CostPeriod::builder()
                    .load(High)
                    .cost(Cost::None)
                    .hours(22, 6)
                    .divide_kw_by(2)
                    .build(),
            ]),
        ))
        .build()])
    .build();
