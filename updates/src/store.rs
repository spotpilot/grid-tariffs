use std::{path::PathBuf, time::Duration};

use convert_case::{Boundary, Case, Casing};
use grid_tariffs::GridOperator;
use tokio::fs::{create_dir_all, read_to_string};
use tracing::{debug, info};

use crate::{
    Country,
    pricing_info::{ContentComparison, ContentFindingResult},
};

#[derive(Debug, Clone)]
pub(crate) struct ResultStore {
    dir: PathBuf,
}

impl ResultStore {
    pub(crate) async fn fetch_and_compare(
        &self,
        operator: &GridOperator,
    ) -> anyhow::Result<ContentComparison> {
        let fetched = self.remote_fetch(operator).await?;
        let stored = self.load(operator).await?;
        let comparison = fetched.compare_to(stored.as_ref());
        if comparison.has_changed() {
            self.store(operator, &fetched).await?;
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
        operator: &GridOperator,
    ) -> anyhow::Result<Option<ContentFindingResult>> {
        let path_full = self.path_full(operator);
        let path_extracted = self.path_extracted(operator);
        if path_full.exists() && path_extracted.exists() {
            let html = read_to_string(path_full).await?;
            let extracted_text = read_to_string(path_extracted).await?;
            debug!(
                grid_operator = operator.name(),
                content_length = html.len(),
                "loaded stored pricing info result"
            );
            Ok(Some(ContentFindingResult::from_generated(
                html,
                extracted_text,
            )))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn remote_fetch(
        &self,
        operator: &GridOperator,
    ) -> anyhow::Result<ContentFindingResult> {
        let html = self.remote_fetch_html(operator).await?;
        Ok(ContentFindingResult::new(operator, html.trim().to_owned())?)
    }

    pub(crate) async fn load_or_remote_fetch(
        &self,
        go: &GridOperator,
    ) -> anyhow::Result<ContentFindingResult> {
        if let Some(stored) = self.load(go).await? {
            Ok(stored)
        } else {
            Ok(self.remote_fetch(go).await?)
        }
    }

    pub(crate) async fn store(
        &self,
        pricing_info: &GridOperator,
        result: &ContentFindingResult,
    ) -> anyhow::Result<()> {
        self.store_full_html(pricing_info, result).await?;
        self.store_extracted(pricing_info, result).await?;
        Ok(())
    }

    pub(crate) async fn store_full_html(
        &self,
        pricing_info: &GridOperator,
        result: &ContentFindingResult,
    ) -> anyhow::Result<()> {
        let path_full = self.path_full(pricing_info);
        tokio::fs::write(path_full, result.html()).await?;
        debug!(
            grid_operator = pricing_info.name(),
            content_length = result.html().len(),
            "stored full html"
        );
        Ok(())
    }

    pub(crate) async fn store_extracted(
        &self,
        pricing_info: &GridOperator,
        result: &ContentFindingResult,
    ) -> anyhow::Result<()> {
        let path_extracted = self.path_extracted(pricing_info);
        tokio::fs::write(path_extracted, result.extracted_content()).await?;
        debug!(
            grid_operator = pricing_info.name(),
            extracted_text = result.extracted_content(),
            "stored extracted text"
        );
        Ok(())
    }

    async fn remote_fetch_html(&self, op: &GridOperator) -> anyhow::Result<String> {
        info!(grid_operator = op.name(), "downloading html...");
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .http1_ignore_invalid_headers_in_responses(true)
            // NOTE: User-Agent is needed to be able to download from EON, otherwise 403
            .user_agent("grid-tariffs/0.1")
            .build()?;
        let req = client.get(op.links().fee_info().link()).build()?;
        let resp = client.execute(req).await?;
        resp.error_for_status_ref()?;
        let text = resp.text().await?;
        Ok(text)
    }

    fn base_path(&self, country: Country) -> PathBuf {
        self.dir.join(country.to_string().to_lowercase())
    }

    fn path_extracted(&self, operator: &GridOperator) -> PathBuf {
        self.base_path(operator.country()).join(format!(
            "{}.extracted.txt",
            Self::operator_filename(operator)
        ))
    }

    fn path_full(&self, operator: &GridOperator) -> PathBuf {
        self.base_path(operator.country())
            .join(format!("{}.full.html", Self::operator_filename(operator)))
    }

    fn operator_filename(operator: &GridOperator) -> String {
        [
            operator.vat_number().to_owned(),
            operator
                .name()
                .trim()
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == ' ')
                .collect::<String>()
                .to_case(Case::Kebab),
        ]
        .join("-")
    }
}
