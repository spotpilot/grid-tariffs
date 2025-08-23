#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    SEK,
}

impl Currency {
    pub const fn sign(&self) -> &'static str {
        match self {
            Self::SEK => "kr",
        }
    }

    pub const fn subunit_sign(&self) -> &'static str {
        match self {
            Self::SEK => "öre",
        }
    }
}
