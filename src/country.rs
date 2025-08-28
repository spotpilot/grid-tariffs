use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Country {
    SE,
}

impl Country {
    pub const fn all() -> &'static [Self] {
        &[Country::SE]
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Country::SE => "Sweden",
        })
    }
}
