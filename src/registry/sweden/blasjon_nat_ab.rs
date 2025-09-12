use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.blasjonnat.se/elnat/59534.natavgifter_2025.html";

pub const BLASJON_NAT_AB: GridOperator = GridOperator::builder()
    .name("Blåsjön Nät AB")
    .vat_number("SE556061917201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 630))
    .links(Links::new(
        Link::builder(FEE_LINK).content_locator_default().build(),
    ))
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(9801, 0).divide_by(12)),
            (20, Money::new(15104, 0).divide_by(12)),
            (25, Money::new(18917, 0).divide_by(12)),
            (35, Money::new(23296, 0).divide_by(12)),
            (50, Money::new(31280, 0).divide_by(12)),
            (63, Money::new(38638, 0).divide_by(12)),
            (80, Money::new(49005, 0).divide_by(12)),
            (100, Money::new(62113, 0).divide_by(12)),
            (125, Money::new(75012, 0).divide_by(12)),
            (160, Money::new(96133, 0).divide_by(12)),
            (200, Money::new(115646, 0).divide_by(12)),
            (250, Money::new(143768, 0).divide_by(12)),
            (315, Money::new(179487, 0).divide_by(12)),
            (355, Money::new(206060, 0).divide_by(12)),
            (400, Money::new(256942, 0).divide_by(12)),
            (500, Money::new(345866, 0).divide_by(12)),
            (630, Money::new(451622, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(27.81))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
