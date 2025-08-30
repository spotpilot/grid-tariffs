use grid_tariffs::{ContentLocator, ContentTarget, LocatorMethod, TargetContainer};
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use tracing::warn;

use crate::helpers::{get_text_with_links_excluding_scripts, remove_unneeded_newlines};

pub(crate) fn locate_content(locator: &ContentLocator, html: &Html) -> String {
    let el_locator = element_locator(locator.method(), html).unwrap();
    let target_container = locator.method().target_container();
    let found = match locator.content_target() {
        ContentTarget::TextWithLinks => el_locator.text_with_links(&target_container),
        ContentTarget::Attribute(attr) => el_locator.attribute_text(attr, &target_container),
    };
    found.unwrap_or_default()
}

pub(crate) fn element_locator<'a>(
    method: &LocatorMethod,
    html: &'a Html,
) -> Option<ElementLocator<'a>> {
    match method {
        LocatorMethod::CssSelector(selector) => {
            ElementLocator::where_selector_matches(html, Selector::parse(selector).unwrap())
        }
        LocatorMethod::TextStartsWith { needle, .. } => {
            ElementLocator::where_text_starts_with(html, needle)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ElementLocator<'a> {
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

    fn where_text_starts_with(document: &'a Html, needle: &'a str) -> Option<Self> {
        let all_selector = Selector::parse("*").unwrap();
        let elements: Vec<ElementRef<'_>> = document
            .select(&all_selector)
            .filter(|el| el.text().collect::<String>().starts_with(needle))
            .collect();
        if elements.is_empty() {
            None
        } else {
            Some(Self { elements })
        }
    }

    /// Get the text nodes of this element, with repeated newlines removed
    fn text_with_links(&self, target_container: &TargetContainer) -> Option<String> {
        let targets = self.locate_targets(target_container);
        if targets.is_empty() {
            warn!("target elements not found");
            return None;
        }
        let text = targets
            .into_iter()
            .map(|t| {
                get_text_with_links_excluding_scripts(t)
                    .iter()
                    .map(|t| t.trim())
                    .join("\n")
            })
            .join("\n\n");
        Some(remove_unneeded_newlines(&text))
    }

    /// Get the `attr` attribute texts of this element, with repeated newlines removed
    fn attribute_text(&self, attr: &str, target_container: &TargetContainer) -> Option<String> {
        let targets = self.locate_targets(target_container);
        if targets.is_empty() {
            warn!("target elements not found");
            return None;
        }
        let text = targets
            .iter()
            .map(|x| {
                x.descendants()
                    .flat_map(|noderef| ElementRef::wrap(noderef))
                    .flat_map(|el| el.attr(attr))
                    .join("\n\n")
            })
            .collect::<Vec<_>>();
        Some(remove_unneeded_newlines(text.join("\n\n").trim()))
    }

    fn locate_targets(&self, target_container: &TargetContainer) -> Vec<ElementRef<'_>> {
        match target_container {
            TargetContainer::Current => self.elements.clone(),
            TargetContainer::Parent => self.nth_ancestor(0).map(|x| vec![x]).unwrap_or_default(),
            TargetContainer::Ancestor(nth) => {
                self.nth_ancestor(*nth).map(|x| vec![x]).unwrap_or_default()
            }
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
