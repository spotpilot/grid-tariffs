use crate::registry::prelude::*;

const FEE_LINK: &str = "https://www.almnas.com/sv/elhandel";

pub static ALMNAS_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Almnäs Elnät AB")
    .vat_number("SE559139372201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 160))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2023, 3, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(7565, 0).divide_by(12)),
            (20, Money::new(11149, 0).divide_by(12)),
            (25, Money::new(13997, 0).divide_by(12)),
            (35, Money::new(18458, 0).divide_by(12)),
            (50, Money::new(25875, 0).divide_by(12)),
            (63, Money::new(33567, 0).divide_by(12)),
            (80, Money::new(43857, 0).divide_by(12)),
            (100, Money::new(54159, 0).divide_by(12)),
            (125, Money::new(62310, 0).divide_by(12)),
            (130, Money::new(64085, 0).divide_by(12)),
            (160, Money::new(79059, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::Unverified)
        .transfer_fee(TransferFee::fixed_subunit(28.43))
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
