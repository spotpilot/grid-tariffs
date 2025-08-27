use std::{path::PathBuf, slice::Iter, time::Duration};

use scraper::Html;
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

impl PricingInfo {
    pub(crate) async fn extract_html(
        &self,
        store_cache: bool,
        load_cache: bool,
    ) -> anyhow::Result<Html> {
        let html = self.extract_raw_html(store_cache, load_cache).await?;
        Ok(Html::parse_document(&html))
    }

    pub(crate) async fn extract_raw_html(
        &self,
        store_cache: bool,
        load_cache: bool,
    ) -> anyhow::Result<String> {
        if load_cache && let Some(html) = self.fetch_cached_html().await? {
            Ok(html)
        } else {
            let html = self.download_html().await?;
            if store_cache {
                self.cache_html(html.as_bytes()).await?;
            }
            Ok(html)
        }
    }

    async fn download_html(&self) -> anyhow::Result<String> {
        info!(grid_operator = self.name, "downloading html...");
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            // NOTE: User-Agent is needed to be able to download from EON, otherwise 403
            .user_agent("grid-tariffs/0.1")
            .build()?;
        let req = client.get(self.link).build()?;
        let resp = client.execute(req).await?;
        resp.error_for_status_ref()?;
        let text = resp.text().await?;
        Ok(text)
    }

    async fn fetch_cached_html(&self) -> anyhow::Result<Option<String>> {
        let path = self
            .html_cache_dir()
            .await?
            .join(self.html_cache_filename());
        if path.exists() {
            let html = read_to_string(path).await?;
            debug!(
                grid_operator = self.name,
                content_length = html.len(),
                "fetched cached html"
            );
            Ok(Some(html))
        } else {
            Ok(None)
        }
    }

    async fn cache_html(&self, html: &[u8]) -> anyhow::Result<()> {
        let path = self
            .html_cache_dir()
            .await?
            .join(self.html_cache_filename());
        tokio::fs::write(path, html).await?;
        debug!(
            grid_operator = self.name,
            content_length = html.len(),
            "cached html"
        );
        Ok(())
    }

    fn html_cache_filename(&self) -> String {
        format!("{}.html", self.name)
    }

    async fn html_cache_dir(&self) -> anyhow::Result<PathBuf> {
        let x = directories::ProjectDirs::from("com", "spotpilot", "grid-tariffs").unwrap();
        let dir = x.cache_dir().join("html");
        create_dir_all(&dir).await?;
        Ok(dir)
    }
}
