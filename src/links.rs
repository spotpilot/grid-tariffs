use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Links {
    /// Website page containing info about the fees that the company charges
    fee_info: Link,
    /// Website page containing info about feed-in revenue
    feed_in_revenue_info: Option<&'static str>,
    /// Link to public Eltariff-API endpoint (https://github.com/RI-SE/Eltariff-API)
    eltariff_api: Option<&'static str>,
}

impl Links {
    pub const fn fee_info(&self) -> &Link {
        &self.fee_info
    }

    pub(crate) const fn new(fee_info: Link) -> Self {
        Self::builder().fee_info(fee_info).build()
    }

    pub(crate) const fn builder() -> LinksBuilder {
        LinksBuilder::new()
    }
}

pub(crate) struct LinksBuilder {
    fee_info: Option<Link>,
    feed_in_revenue_info: Option<&'static str>,
    eltariff_api: Option<&'static str>,
}

impl LinksBuilder {
    pub(crate) const fn new() -> Self {
        Self {
            fee_info: None,
            feed_in_revenue_info: None,
            eltariff_api: None,
        }
    }

    pub(crate) const fn fee_info(mut self, link: Link) -> Self {
        self.fee_info = Some(link);
        self
    }

    pub(crate) const fn feed_in_revenue_info(mut self, link: &'static str) -> Self {
        self.feed_in_revenue_info = Some(link);
        self
    }

    pub(crate) const fn eltariff_api(mut self, link: &'static str) -> Self {
        self.eltariff_api = Some(link);
        self
    }

    pub(crate) const fn build(self) -> Links {
        Links {
            fee_info: self.fee_info.expect("`fee_info` not specified"),
            feed_in_revenue_info: self.feed_in_revenue_info,
            eltariff_api: self.eltariff_api,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TargetContainer {
    Current,
    Parent,
    Ancestor(usize),
}

/// What content to checksum witihin the elements found
#[derive(Debug, Clone, Serialize)]
pub enum ContentTarget {
    TextWithLinks,
    /// Attribute which contains the relevant content
    Attribute(&'static str),
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentLocator {
    method: LocatorMethod,
    content: ContentTarget,
    uses_default_locator: bool,
}

impl ContentLocator {
    pub(crate) const fn new(method: LocatorMethod, content: ContentTarget) -> Self {
        Self {
            method,
            content,
            uses_default_locator: false,
        }
    }

    pub const fn method(&self) -> &LocatorMethod {
        &self.method
    }

    pub const fn uses_default_locator(&self) -> bool {
        self.uses_default_locator
    }

    pub const fn content_target(&self) -> &ContentTarget {
        &self.content
    }

    pub(crate) const fn new_starts_with(
        needle: &'static str,
        target_container: TargetContainer,
        content: ContentTarget,
    ) -> ContentLocator {
        Self::new(
            LocatorMethod::TextStartsWith {
                needle,
                target_container,
            },
            content,
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum LocatorMethod {
    CssSelector(&'static str),
    TextStartsWith {
        needle: &'static str,
        target_container: TargetContainer,
    },
}

impl LocatorMethod {
    pub fn target_container(&self) -> TargetContainer {
        match self {
            Self::CssSelector(_) => TargetContainer::Current,
            Self::TextStartsWith {
                target_container, ..
            } => *target_container,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    link: &'static str,
    content_locator: ContentLocator,
}

impl Link {
    pub const fn link(&self) -> &str {
        &self.link
    }

    pub const fn content_locator(&self) -> &ContentLocator {
        &self.content_locator
    }

    pub(crate) const fn builder(link: &'static str) -> LinkBuilder {
        LinkBuilder::new(link)
    }
}

pub struct LinkBuilder {
    link: &'static str,
    content_locator: Option<ContentLocator>,
}

impl LinkBuilder {
    pub(crate) const fn new(link: &'static str) -> Self {
        Self {
            link,
            content_locator: None,
        }
    }

    pub(crate) const fn content_locator(mut self, locator: ContentLocator) -> Self {
        self.content_locator = Some(locator);
        self
    }

    pub(crate) const fn plain_content_locator(mut self, css_selector: &'static str) -> Self {
        self.content_locator = Some(ContentLocator::new(
            LocatorMethod::CssSelector(css_selector),
            ContentTarget::TextWithLinks,
        ));
        self
    }

    pub(crate) const fn content_locator_default(mut self) -> Self {
        self.content_locator = Some(ContentLocator {
            method: LocatorMethod::CssSelector("main"),
            content: ContentTarget::TextWithLinks,
            uses_default_locator: true,
        });
        self
    }

    pub(crate) const fn build(self) -> Link {
        Link {
            link: self.link,
            content_locator: self.content_locator.expect("`locator` missing"),
        }
    }
}
