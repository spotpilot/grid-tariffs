use crate::registry::prelude::*;

const FEE_LINK: &str =
    "https://ystadenergi.se/ystad-energi/elnatet-i-ystad/priser-och-avtal/privat-och-sma-foretag";

const BASE_PRICELIST: PriceListBuilder = PriceListBuilder::new()
    .monthly_fee(Cost::fuses(&[
        (16, Money::new(472, 30)),
        (20, Money::new(623, 50)),
        (25, Money::new(819, 90)),
        (35, Money::new(1323, 40)),
        (50, Money::new(1968, 00)),
        (63, Money::new(2759, 20)),
    ]))
    .monthly_production_fee(Cost::fixed(114, 60))
    .feed_in_revenue(FeedInRevenue::spot_price_variable(
        Country::SE.add_vat(2.41 + 0.96),
        0.05,
        false,
    ))
    .transfer_fee(TransferFee::spot_price_variable(
        Country::SE.add_vat(2.41 + 2.96),
        0.05,
        false,
    ))
    .power_tariff(PowerTariff::NotImplemented);

pub static YSTAD_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Ystad Elnät AB")
    .vat_number("SE559408185201")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(Links::builder().fee_info_default(FEE_LINK).build())
    .price_lists(&[
        BASE_PRICELIST.from_date(2025, 7, 1).build(),
        // October contains the same pricelist, but with 15 minute spot prices
        BASE_PRICELIST.from_date(2025, 10, 1).build(),
    ])
    .build();
