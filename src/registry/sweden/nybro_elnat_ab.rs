use crate::registry::prelude::*;

const FEE_LINK: &str = "https://nybroenergi.se/vara-tjanster/el/prislista-elnat/";

pub static NYBRO_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Nybro Elnät AB")
    .vat_number("SE556058489701")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 200))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(3971, 82).divide_by(12)),
            (20, Money::new(6202, 67).divide_by(12)),
            (25, Money::new(7844, 5).divide_by(12)),
            (35, Money::new(13158, 17).divide_by(12)),
            (50, Money::new(18381, 59).divide_by(12)),
            (63, Money::new(23886, 14).divide_by(12)),
            (80, Money::new(31378, 11).divide_by(12)),
            (100, Money::new(40608, 34).divide_by(12)),
            (125, Money::new(50257, 18).divide_by(12)),
            (160, Money::new(63769, 16).divide_by(12)),
            (200, Money::new(82794, 76).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::Unverified)
        .feed_in_revenue(FeedInRevenue::fixed_subunit(5.0))
        // "Den 1 januari 2024 övergick vi till en ny prismodell med en rörlig överföringsavgift. Detta är en direkt följd av Svenska kraftnäts ändrade prismodell från 2020. Överföringsavgiften i den nya prismodellen kommer att variera månad för månad beroende på genomsnittet av Norpools spotpris för innevarande månad."
        // TODO: Fix once we have correct figures
        .transfer_fee(TransferFee::spot_price_variable_placeholder())
        .power_tariff(PowerTariff::NotImplemented)
        .build()])
    .build();
