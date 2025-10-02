use crate::registry::prelude::*;

const FEE_INFO_LINK: &str = "https://trelleborgsenergi.se/el-som-gor-skillnad/elnat/elnatsavgift/elnatsavgifter-for-sakringskunder/";
const FEED_IN_REVENUE_LINK: &str =
    "https://trelleborgsenergi.se/el-som-gor-skillnad/vara-elpriser/";

pub static TRELLEBORG_ELNAT_AB: GridOperator = GridOperator::builder()
    .name("Trelleborg Elnät AB")
    .vat_number("SE559181616901")
    .country(Country::SE)
    .main_fuses(MainFuseSizes::new_range(16, 63))
    .links(
        Links::builder()
            .fee_info(FEE_INFO_LINK, "table")
            .feed_in_revenue_info(FEED_IN_REVENUE_LINK, ".wp-block-group:nth-child(3) table")
            .build(),
    )
    .price_lists(&[PriceList::builder()
        .from_date(2025, 1, 1)
        .monthly_fee(Cost::fuses(&[
            (16, Money::new(5_301, 0).divide_by(12)),
            (20, Money::new(6_880, 0).divide_by(12)),
            (25, Money::new(8_243, 0).divide_by(12)),
            (35, Money::new(12_211, 0).divide_by(12)),
            (50, Money::new(18_906, 0).divide_by(12)),
            (63, Money::new(23_476, 0).divide_by(12)),
            (80, Money::new(30_346, 0).divide_by(12)),
            (100, Money::new(39_014, 0).divide_by(12)),
            (125, Money::new(50_396, 0).divide_by(12)),
            (160, Money::new(65_174, 0).divide_by(12)),
            (200, Money::new(85_025, 0).divide_by(12)),
        ]))
        .monthly_production_fee(Cost::fixed(375, 0).divide_by(12))
        .feed_in_revenue(FeedInRevenue::Unverified)
        // NOTE: Not really fixed, changes each month. "De rörliga kostnaderna kommer att variera med månadssnittet av elpriset på elbörsen Nord Pool."
        .transfer_fee(TransferFee::fixed_subunit(12.09))
        .power_tariff(PowerTariff::Unverified)
        .build()])
    .build();
