use std::io::stdout;

use chrono::{TimeDelta, Utc};
use csv::WriterBuilder;
use grid_tariffs::{GridOperator, Link, Links};
use itertools::Itertools;

pub(crate) fn report() -> anyhow::Result<()> {
    #[derive(Debug, serde::Serialize)]
    struct ReportRow {
        name: String,
        completion_percentage: u8,
        monthly_fee: bool,
        monthly_production_fee: bool,
        transfer_fee: bool,
        feed_in_revenue: bool,
        other_fees: bool,
        power_tariff: bool,
        fee_info_filled_out: bool,
        fee_info_uses_default_locator: bool,
    }

    let rows = GridOperator::all()
        .iter()
        .map(|op| {
            let stats = FilledOutStats::new(op);
            ReportRow {
                name: op.name().to_owned(),
                completion_percentage: stats.completion_percentage,
                monthly_fee: stats.monthly_fee,
                monthly_production_fee: stats.monthly_production_fee,
                transfer_fee: stats.transfer_fee,
                feed_in_revenue: stats.feed_in_revenue,
                other_fees: stats.other_fees,
                power_tariff: stats.power_tariff,
                fee_info_filled_out: stats.links.fee_info.filled_out,
                fee_info_uses_default_locator: stats.links.fee_info.uses_default_locator,
            }
        })
        .sorted_by_key(|row| 100 - row.completion_percentage)
        .collect_vec();
    let mut wtr = WriterBuilder::new().from_writer(stdout());
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct FilledOutStats {
    price_date: bool,
    monthly_fee: bool,
    monthly_production_fee: bool,
    transfer_fee: bool,
    feed_in_revenue: bool,
    other_fees: bool,
    power_tariff: bool,
    links: LinksStats,
    completion_percentage: u8,
}

impl FilledOutStats {
    fn new(operator: &GridOperator) -> Self {
        let mut this = Self::default();
        if (Utc::now().date_naive() - operator.price_date()).abs() < TimeDelta::days(3650) {
            this.price_date = true;
        }
        this.monthly_fee = !operator.monthly_fee().is_unverified();
        this.monthly_production_fee = !operator.monthly_production_fee().is_unverified();
        this.transfer_fee = !operator.transfer_fee().is_unverified();
        this.feed_in_revenue = !operator.feed_in_revenue().is_unverified();
        this.other_fees = !operator.other_fees().is_unverified();
        this.power_tariff = !operator.power_tariff().is_unverified();
        this.links = LinksStats::new(operator.links());
        this.completion_percentage = this.calculate_completion_percentage();
        this
    }

    fn calculate_completion_percentage(&self) -> u8 {
        let mut percentage = 0;
        if self.power_tariff {
            percentage += 20;
        }
        if self.transfer_fee {
            percentage += 20;
        }
        if self.price_date {
            percentage += 10;
        }
        if self.monthly_fee {
            percentage += 10;
        }
        if self.monthly_production_fee {
            percentage += 5;
        }
        if self.feed_in_revenue {
            percentage += 10;
        }
        if self.other_fees {
            percentage += 5;
        }
        if self.links.fee_info.filled_out {
            percentage += 10;
        }
        if !self.links.fee_info.uses_default_locator {
            percentage += 10;
        }
        percentage
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct LinksStats {
    /// Fee info filled out
    fee_info: LinkStats,
}

impl LinksStats {
    fn new(links: &Links) -> Self {
        Self {
            fee_info: LinkStats::new(links.fee_info()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LinkStats {
    filled_out: bool,
    uses_default_locator: bool,
}

impl LinkStats {
    fn new(link: &Link) -> Self {
        Self {
            filled_out: !link.link().is_empty(),
            uses_default_locator: link.content_locator().uses_default_locator(),
        }
    }
}

impl Default for LinkStats {
    fn default() -> Self {
        Self {
            filled_out: false,
            uses_default_locator: true,
        }
    }
}
