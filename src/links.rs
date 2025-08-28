#[derive(Debug, Clone)]
pub struct Links {
    /// Website page containing info about the fees that the company charges
    fee_info: &'static str,
    /// Website page containing info about feed-in revenue
    feed_in_revenue_info: Option<&'static str>,
    /// Link to public Eltariff-API endpoint (https://github.com/RI-SE/Eltariff-API)
    eltariff_api: Option<&'static str>,
}

impl Links {
    pub const fn fee_info(&self) -> &str {
        self.fee_info
    }

    pub(crate) const fn new(fee_info: &'static str) -> Self {
        Self::builder().fee_info(fee_info).build()
    }

    pub(crate) const fn builder() -> LinksBuilder {
        LinksBuilder::new()
    }
}

pub(crate) struct LinksBuilder {
    fee_info: Option<&'static str>,
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

    pub(crate) const fn fee_info(mut self, link: &'static str) -> Self {
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
