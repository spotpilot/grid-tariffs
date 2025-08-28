use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Country {
    SE,
}

impl Country {
    pub const fn all() -> &'static [Self] {
        &[Country::SE]
    }
}

impl FromStr for Country {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_ref() {
            "SE" => Ok(Country::SE),
            _ => Err("no such country"),
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Country::SE => "Sweden",
        })
    }
}
