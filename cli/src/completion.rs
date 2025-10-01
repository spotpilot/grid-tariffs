use std::{path::Path, str::FromStr};

use chrono::{TimeDelta, Utc};
use csv::WriterBuilder;
use grid_tariffs::{GridOperator, Link, Links};
use itertools::Itertools;
use rust_xlsxwriter::{Table, workbook::Workbook};
use tracing::info;

#[derive(Debug, Clone, Copy)]
pub(crate) enum OutputFormat {
    Xlsx,
    Csv,
}

impl OutputFormat {
    fn file_extension(&self) -> &'static str {
        match self {
            OutputFormat::Xlsx => "xlsx",
            OutputFormat::Csv => "csv",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "xlsx" => Ok(Self::Xlsx),
            "csv" => Ok(Self::Csv),
            _ => Err("No such output format"),
        }
    }
}

pub(crate) fn report(format: OutputFormat, output_dir: &Path) -> anyhow::Result<()> {
    #[derive(Debug, serde::Serialize)]
    struct ReportRow {
        name: String,
        vat_number: String,
        completion_percentage: u8,
        monthly_fee: bool,
        monthly_production_fee: bool,
        transfer_fee: bool,
        feed_in_revenue: bool,
        power_tariff: bool,
        fee_info_filled_out: bool,
        fee_info_explicit_locator: bool,
    }

    let rows = GridOperator::all()
        .iter()
        .flat_map(|op| {
            FilledOutStats::new(op).into_iter().map(|stats| ReportRow {
                name: op.name().to_owned(),
                vat_number: op.vat_number().to_owned(),
                completion_percentage: stats.completion_percentage,
                monthly_fee: stats.monthly_fee,
                monthly_production_fee: stats.monthly_production_fee,
                transfer_fee: stats.transfer_fee,
                feed_in_revenue: stats.feed_in_revenue,
                power_tariff: stats.power_tariff,
                fee_info_filled_out: stats.links.fee_info.filled_out,
                fee_info_explicit_locator: !stats.links.fee_info.uses_default_locator,
            })
        })
        .sorted_by_key(|row| row.completion_percentage)
        .collect_vec();

    let filename = format!(
        "grid-tariffs-completion-report-{}.{}",
        Utc::now().format("%Y-%m-%d-%H.%M"),
        format.file_extension()
    );
    let out_path = output_dir.join(filename);
    info!(
        "Saving completion report to: {}",
        out_path.to_string_lossy()
    );
    match format {
        OutputFormat::Xlsx => {
            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();

            if let Some(row) = rows.first() {
                worksheet.serialize_headers(0, 0, row)?;
            }

            worksheet.serialize(&rows)?;

            let table = Table::new();
            worksheet.add_table(0, 0, (rows.len() - 1) as u32, 10, &table)?;
            workbook.save(out_path)?;
        }
        OutputFormat::Csv => {
            let mut wtr = WriterBuilder::new().from_path(out_path)?;
            for row in rows {
                wtr.serialize(row)?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct FilledOutStats {
    price_date: bool,
    monthly_fee: bool,
    monthly_production_fee: bool,
    transfer_fee: bool,
    feed_in_revenue: bool,
    power_tariff: bool,
    links: LinksStats,
    completion_percentage: u8,
}

impl FilledOutStats {
    fn new(operator: &GridOperator) -> Vec<Self> {
        let mut ret = Vec::new();
        for pl in operator.price_lists() {
            let mut this = Self::default();
            if (Utc::now().date_naive() - pl.from_date()).abs() < TimeDelta::days(3650) {
                this.price_date = true;
            }
            this.monthly_fee = !pl.monthly_fee().is_unverified();
            this.monthly_production_fee = !pl.monthly_production_fee().is_unverified();
            this.transfer_fee = !pl.transfer_fee().is_unverified();
            this.feed_in_revenue = !pl.feed_in_revenue().is_unverified();
            this.power_tariff = !pl.power_tariff().is_unverified();
            this.links = LinksStats::new(operator.links());
            this.completion_percentage = this.calculate_completion_percentage();
            ret.push(this);
        }
        ret
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
        if self.feed_in_revenue {
            percentage += 10;
        }
        if self.links.fee_info.filled_out {
            percentage += 10;
        }
        if !self.links.fee_info.uses_default_locator {
            percentage += 10;
        }
        if self.monthly_production_fee {
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
