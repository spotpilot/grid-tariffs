use std::{path::PathBuf, slice::Iter, time::Duration};

use scraper::Html;
use serde::{Deserialize, Serialize};
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
    old: Option<String>,
    new: Option<String>,
}

impl ChangedPricing {
    fn new(old: Option<String>, new: Option<String>) -> Self {
        Self { old, new }
    }

    fn diff(&self) -> String {
        let diff = TextDiff::from_lines(
            self.old.as_deref().unwrap_or_default(),
            self.new.as_deref().unwrap_or_default(),
        );
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

#[derive(Serialize, Deserialize)]
pub(crate) struct PricingInfoResult {
    html: String,
    extracted_text: Option<String>,
}

impl PricingInfoResult {
    pub(crate) fn html(&self) -> &str {
        &self.html
    }

    pub(crate) fn extracted_text(&self) -> Option<&str> {
        self.extracted_text.as_deref()
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
                self.extracted_text().map(str::to_owned),
                self.extracted_text().map(str::to_owned),
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
        create_dir_all(&results_dir)
            .await
            .expect("creating result dir should work");
        Self { dir: results_dir }
    }

    pub(crate) async fn load(&self, pi: &PricingInfo) -> anyhow::Result<Option<PricingInfoResult>> {
        let path = self.dir.join(format!("{}.html", pi.name));
        if path.exists() {
            let stored = read_to_string(path).await?;
            let res: PricingInfoResult = serde_json::from_str(&stored)?;
            debug!(
                grid_operator = pi.name,
                content_length = res.html().len(),
                "loaded stored pricing info result"
            );
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn remote_fetch(&self, pi: &PricingInfo) -> anyhow::Result<PricingInfoResult> {
        let html = self.fetch_html(pi).await?;
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
        let path = self.dir.join(format!("{}.html", pricing_info.name));
        let serialized = serde_json::to_string_pretty(result)?;
        tokio::fs::write(path, &serialized).await?;
        debug!(
            grid_operator = pricing_info.name,
            result = serialized,
            "stored pricing info result"
        );
        Ok(())
    }

    async fn fetch_html(&self, pi: &PricingInfo) -> anyhow::Result<String> {
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
}
