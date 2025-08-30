use core::fmt;

use grid_tariffs::GridOperator;
use scraper::Html;
use similar::TextDiff;

use crate::locator::locate_content;

#[derive(Debug, Clone)]
pub(crate) enum ContentComparison {
    Unchanged,
    New,
    #[allow(dead_code)]
    ChangedPricing(ChangedPricing),
}

impl fmt::Display for ContentComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            ContentComparison::Unchanged => "unchanged".to_string(),
            ContentComparison::New => "new".to_string(),
            ContentComparison::ChangedPricing(changed_pricing) => format!(
                "changed old({} chars) -> new({} chars)",
                changed_pricing.old.char_indices().count(),
                changed_pricing.new.char_indices().count()
            ),
        })
    }
}

impl ContentComparison {
    pub(crate) fn diff(&self) -> String {
        match self {
            ContentComparison::Unchanged => "".into(),
            ContentComparison::New => "[new]".into(),
            ContentComparison::ChangedPricing(changed_pricing) => changed_pricing.diff(),
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
    fn diff(&self) -> String {
        let diff = TextDiff::from_lines(&self.old, &self.new);
        diff.unified_diff().to_string()
    }
}

impl ContentComparison {
    pub(super) fn has_changed(&self) -> bool {
        match self {
            ContentComparison::Unchanged => false,
            ContentComparison::New | ContentComparison::ChangedPricing(..) => true,
        }
    }
}

pub(crate) struct ContentFindingResult {
    html: String,
    extracted_content: String,
}

impl ContentFindingResult {
    pub(crate) fn html(&self) -> &str {
        &self.html
    }

    pub(crate) fn extracted_content(&self) -> &str {
        &self.extracted_content
    }

    pub(super) fn new(operator: &GridOperator, html: String) -> Self {
        let parsed_html = Html::parse_document(&html);
        let locator = operator.links().fee_info().content_locator();
        let extracted_content = locate_content(locator, &parsed_html);
        Self {
            html,
            extracted_content,
        }
    }

    pub(crate) fn from_generated(html: String, extracted_content: String) -> ContentFindingResult {
        Self {
            html,
            extracted_content,
        }
    }

    pub(super) fn compare_to(&self, other: Option<&ContentFindingResult>) -> ContentComparison {
        let Some(other) = other else {
            return ContentComparison::New;
        };
        if self.extracted_content == other.extracted_content {
            ContentComparison::Unchanged
        } else {
            ContentComparison::ChangedPricing(ChangedPricing {
                new: self.extracted_content.clone(),
                old: other.extracted_content.clone(),
            })
        }
    }
}
