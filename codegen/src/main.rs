use std::{path::PathBuf, time::Duration};
use tokio::fs::{create_dir_all, read_to_string};
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt,
    prelude::*,
};

use LocatorMethod::*;
use clap::Parser;
use scraper::{ElementRef, Html, Selector};
use tracing::{debug, info, warn};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'l', long, env, default_value = "INFO")]
    log_level: LevelFilter,
    #[arg(long)]
    store_cache: bool,
    #[arg(long)]
    use_cached: bool,
    grid_operator: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let targets = Targets::new()
        .with_default(cli.log_level)
        .with_target("html5ever", LevelFilter::INFO)
        .with_target("h2", LevelFilter::INFO)
        .with_target("hyper_util", LevelFilter::INFO)
        .with_target("selectors", LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(targets)
        .init();

    let Some(gi) = PRICING_INFO
        .iter()
        .find(|pi| pi.name.to_ascii_lowercase() == cli.grid_operator.to_ascii_lowercase())
    else {
        anyhow::bail!("Grid operator not found");
    };
    let html = if cli.use_cached
        && let Some(html) = gi.fetch_cached_html().await?
    {
        html
    } else {
        let html = gi.download_html().await?;
        if cli.store_cache {
            gi.cache_html(html.as_bytes()).await?;
        }
        html
    };
    let html = Html::parse_document(&html);
    let text = gi.locator.locate_content(&html);
    info!("{}", text.unwrap_or_default());
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum TargetContainer {
    Current,
    Parent,
    Ancestor(usize),
}

/// What content to checksum witihin the elements found
#[derive(Debug, Clone)]
enum ContentTarget {
    Text,
    /// Attribute which contains the relevant content
    Attribute(&'static str),
}

#[derive(Debug, Clone)]
struct Locator {
    method: LocatorMethod,
    content: ContentTarget,
}
impl Locator {
    const fn new(method: LocatorMethod, content: ContentTarget) -> Self {
        Self { method, content }
    }

    fn locate_content(&self, html: &Html) -> Option<String> {
        let found = match self.content {
            ContentTarget::Text => self.method.locate_text(html),
            ContentTarget::Attribute(attr) => self.method.locate_attribute_text(html, attr),
        };
        found
    }
}

#[derive(Debug, Clone)]
enum LocatorMethod {
    CssSelector(&'static str),
    TextStartsWith {
        needle: &'static str,
        target_container: TargetContainer,
    },
}

impl LocatorMethod {
    fn element_locator<'a>(&self, html: &'a Html) -> Option<ElementLocator<'a>> {
        match self {
            CssSelector(selector) => {
                ElementLocator::where_selector_matches(html, Selector::parse(selector).unwrap())
            }
            TextStartsWith { needle, .. } => ElementLocator::where_text_starts_with(html, needle),
        }
    }

    fn target_container(&self) -> TargetContainer {
        match self {
            CssSelector(_) => TargetContainer::Current,
            TextStartsWith {
                target_container, ..
            } => *target_container,
        }
    }

    fn locate_text(&self, html: &Html) -> Option<String> {
        let Some(locator) = self.element_locator(html) else {
            warn!("failed to get element locator");
            return None;
        };
        locator.text(&self.target_container())
    }

    fn locate_attribute_text(&self, html: &Html, attr: &str) -> Option<String> {
        let Some(locator) = self.element_locator(html) else {
            warn!("failed to get element locator");
            return None;
        };
        locator.attribute_text(attr, &self.target_container())
    }
}

#[derive(Debug, Clone)]
struct PricingInfo {
    name: &'static str,
    link: &'static str,
    locator: Locator,
}

impl PricingInfo {
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

    fn html_cache_filename(&self) -> String {
        format!("{}.html", self.name)
    }

    async fn html_cache_dir(&self) -> anyhow::Result<PathBuf> {
        let x = directories::ProjectDirs::from("com", "spotpilot", "grid-tariffs").unwrap();
        let dir = x.cache_dir().join("html");
        create_dir_all(&dir).await?;
        Ok(dir)
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
}

static PRICING_INFO: &[PricingInfo] = &[
    PricingInfo {
        name: "EON",
        link: "https://www.eon.se/el/elnat/elnaetsabonnemang-priser",
        // NOTE: At the time of writing there are four tables with pricing info. Three are current and one is with old Stockholm prices.
        locator: Locator::new(
            CssSelector(r#"eon-ui-table-renderer"#),
            ContentTarget::Attribute("content"),
        ),
    },
    PricingInfo {
        name: "Ellevio",
        link: "https://www.ellevio.se/abonnemang/elnatspriser-privat/",
        locator: Locator::new(
            TextStartsWith {
                needle: "För bland annat villor, radhus, fritidshus och verksamhetslokaler med egen anslutning till elnätet – ej lägenheter.",
                target_container: TargetContainer::Parent,
            },
            ContentTarget::Text,
        ),
    },
    PricingInfo {
        name: "Vattenfall",
        link: "https://www.vattenfalleldistribution.se/abonnemang-och-avgifter/avtal-och-avgifter/elnatsavgift-och-avtalsvillkor/",
        locator: Locator::new(
            TextStartsWith {
                needle: "Säkringsabonnemang (16–63 A)",
                target_container: TargetContainer::Ancestor(1),
            },
            // CssSelector("[data-content]"),
            ContentTarget::Attribute("data-content"),
        ),
    },
];

#[derive(Debug)]
struct ElementLocator<'a> {
    elements: Vec<ElementRef<'a>>,
}

impl<'a> ElementLocator<'a> {
    fn where_selector_matches<'b>(document: &'a Html, selector: Selector) -> Option<Self> {
        let elements: Vec<ElementRef<'a>> = document.select(&selector).collect();
        if elements.is_empty() {
            None
        } else {
            Some(Self { elements })
        }
    }

    fn where_text_starts_with(html: &'a Html, needle: &'a str) -> Option<Self> {
        let elements = find_text_elements(html, needle);
        if elements.is_empty() {
            None
        } else {
            Some(Self { elements })
        }
    }

    /// Get the text nodes of this element, with repeated newlines removed
    fn text(&self, target_container: &TargetContainer) -> Option<String> {
        if let Some(target) = self.locate_target(target_container) {
            let text = target.text().collect::<String>();
            Some(remove_unneeded_newlines(text.trim()))
        } else {
            None
        }
    }

    /// Get the `attr` attribute texts of this element, with repeated newlines removed
    fn attribute_text(&self, attr: &str, target_container: &TargetContainer) -> Option<String> {
        if let Some(target) = self.locate_target(target_container) {
            let text = target
                .descendants()
                .flat_map(|noderef| ElementRef::wrap(noderef))
                .flat_map(|el| el.attr(attr))
                .collect::<String>();
            Some(remove_unneeded_newlines(text.trim()))
        } else {
            None
        }
    }

    fn locate_target(&self, target_container: &TargetContainer) -> Option<ElementRef<'_>> {
        match target_container {
            TargetContainer::Current => None,
            TargetContainer::Parent => self.nth_ancestor(0),
            TargetContainer::Ancestor(nth) => self.nth_ancestor(*nth),
        }
    }

    fn nth_ancestor(&self, nth: usize) -> Option<ElementRef<'_>> {
        self.elements
            .iter()
            .map(|el| el.ancestors().skip(nth))
            .flatten()
            .next()
            .map(ElementRef::wrap)
            .flatten()
    }
}

fn find_text_elements<'a, 'b>(document: &'a Html, target_text: &'b str) -> Vec<ElementRef<'a>> {
    let all_selector = Selector::parse("*").unwrap();
    document
        .select(&all_selector)
        .filter(|el| el.text().collect::<String>().starts_with(target_text))
        .collect()
}

fn remove_unneeded_newlines(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut num_empty = 0;
    for line in lines {
        if line.is_empty() {
            num_empty += 1;
        } else {
            if num_empty > 1 {
                result.push("");
            }
            result.push(line);
            num_empty = 0;
        }
    }
    result.join("\n")
}
