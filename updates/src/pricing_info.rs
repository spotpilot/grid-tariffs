use core::fmt;
use std::{path::PathBuf, slice::Iter, time::Duration};

use grid_tariffs::Country;
use scraper::Html;
use similar::TextDiff;
use tokio::fs::{create_dir_all, read_to_string};
use tracing::{debug, info};

use crate::locator::Locator;

#[derive(Debug, Clone)]
pub(crate) struct PricingInfoRegistry<'a>(&'a [PricingInfo]);

impl<'a> PricingInfoRegistry<'a> {
    pub(crate) const fn new(items: &'a [PricingInfo]) -> Self {
        Self(items)
    }

    pub(crate) fn get(&self, name: &str) -> Option<&'a PricingInfo> {
        self.iter()
            .find(|pi| pi.name.to_ascii_lowercase() == name.to_ascii_lowercase())
    }

    pub(crate) fn iter(&self) -> Iter<'a, PricingInfo> {
        self.0.iter()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PricingInfo {
    pub(crate) name: &'static str,
    pub(crate) country: Country,
    pub(crate) link: &'static str,
    pub(crate) locator: Locator,
}

#[derive(Debug, Clone)]
pub(crate) enum PricingInfoComparison {
    Unchanged,
    New,
    #[allow(dead_code)]
    ChangedPricing(ChangedPricing),
}

impl fmt::Display for PricingInfoComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            PricingInfoComparison::Unchanged => "unchanged".to_string(),
            PricingInfoComparison::New => "new".to_string(),
            PricingInfoComparison::ChangedPricing(changed_pricing) => format!(
                "changed old({} chars) -> new({} chars)",
                changed_pricing.old.char_indices().count(),
                changed_pricing.new.char_indices().count()
            ),
        })
    }
}

impl PricingInfoComparison {
    pub(crate) fn diff(&self) -> String {
        match self {
            PricingInfoComparison::Unchanged => "".into(),
            PricingInfoComparison::New => "[new]".into(),
            PricingInfoComparison::ChangedPricing(changed_pricing) => changed_pricing.diff(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct ChangedPricing {
    old: String,
    new: String,
}

impl ChangedPricing {
    fn new(old: String, new: String) -> Self {
        Self { old, new }
    }

    fn diff(&self) -> String {
        let diff = TextDiff::from_lines(&self.old, &self.new);
        diff.unified_diff().to_string()
    }
}

impl PricingInfoComparison {
    fn has_changed(&self) -> bool {
        match self {
            PricingInfoComparison::Unchanged => false,
            PricingInfoComparison::New | PricingInfoComparison::ChangedPricing(..) => true,
        }
    }
}

pub(crate) struct PricingInfoResult {
    html: String,
    extracted_text: String,
}

impl PricingInfoResult {
    pub(crate) fn html(&self) -> &str {
        &self.html
    }

    pub(crate) fn extracted_text(&self) -> &str {
        &self.extracted_text
    }

    fn new(pricing_info: &PricingInfo, html: String) -> Self {
        let parsed_html = Html::parse_document(&html);
        let extracted_text = pricing_info.locator.locate_content(&parsed_html);
        Self {
            html,
            extracted_text,
        }
    }

    fn compare_to(&self, other: Option<&PricingInfoResult>) -> PricingInfoComparison {
        let Some(other) = other else {
            return PricingInfoComparison::New;
        };
        if self.extracted_text == other.extracted_text {
            PricingInfoComparison::Unchanged
        } else {
            PricingInfoComparison::ChangedPricing(ChangedPricing::new(
                self.extracted_text.clone(),
                other.extracted_text.clone(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ResultStore {
    dir: PathBuf,
}

impl ResultStore {
    pub(crate) async fn fetch_and_compare(
        &self,
        pricing_info: &PricingInfo,
    ) -> anyhow::Result<PricingInfoComparison> {
        let fetched = self.remote_fetch(pricing_info).await?;
        let stored = self.load(pricing_info).await?;
        let comparison = fetched.compare_to(stored.as_ref());
        if comparison.has_changed() {
            self.store(pricing_info, &fetched).await?;
        }
        Ok(comparison)
    }

    pub(crate) async fn new(results_dir: PathBuf) -> Self {
        let self_ = Self { dir: results_dir };
        for country in Country::all() {
            create_dir_all(&self_.base_path(*country))
                .await
                .expect("creating result dir should work");
        }
        self_
    }

    pub(crate) async fn load(
        &self,
        pricing_info: &PricingInfo,
    ) -> anyhow::Result<Option<PricingInfoResult>> {
        let path_full = self.path_full(pricing_info);
        let path_extracted = self.path_extracted(pricing_info);
        if path_full.exists() && path_extracted.exists() {
            let html = read_to_string(path_full).await?;
            debug!(
                grid_operator = pricing_info.name,
                content_length = html.len(),
                "loaded stored pricing info result"
            );
            Ok(Some(PricingInfoResult::new(pricing_info, html)))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn remote_fetch(&self, pi: &PricingInfo) -> anyhow::Result<PricingInfoResult> {
        let html = self.remote_fetch_html(pi).await?;
        Ok(PricingInfoResult::new(pi, html))
    }

    pub(crate) async fn load_or_remote_fetch(
        &self,
        pi: &PricingInfo,
    ) -> anyhow::Result<PricingInfoResult> {
        if let Some(stored) = self.load(pi).await? {
            Ok(stored)
        } else {
            Ok(self.remote_fetch(pi).await?)
        }
    }

    pub(crate) async fn store(
        &self,
        pricing_info: &PricingInfo,
        result: &PricingInfoResult,
    ) -> anyhow::Result<()> {
        let path_full = self.path_full(pricing_info);
        let path_extracted = self.path_extracted(pricing_info);
        tokio::fs::write(path_full, result.html()).await?;
        tokio::fs::write(path_extracted, result.extracted_text()).await?;
        debug!(
            grid_operator = pricing_info.name,
            extracted_text = result.extracted_text(),
            content_length = result.html().len(),
            "stored pricing info result"
        );
        Ok(())
    }

    async fn remote_fetch_html(&self, pi: &PricingInfo) -> anyhow::Result<String> {
        info!(grid_operator = pi.name, "downloading html...");
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            // NOTE: User-Agent is needed to be able to download from EON, otherwise 403
            .user_agent("grid-tariffs/0.1")
            .build()?;
        let req = client.get(pi.link).build()?;
        let resp = client.execute(req).await?;
        resp.error_for_status_ref()?;
        let text = resp.text().await?;
        Ok(text)
    }

    fn base_path(&self, country: Country) -> PathBuf {
        self.dir.join(country.to_string().to_lowercase())
    }

    fn path_extracted(&self, pricing_info: &PricingInfo) -> PathBuf {
        self.base_path(pricing_info.country)
            .join(format!("{}.extracted.txt", pricing_info.name))
    }

    fn path_full(&self, pricing_info: &PricingInfo) -> PathBuf {
        self.base_path(pricing_info.country)
            .join(format!("{}.full.html", pricing_info.name))
    }
}
