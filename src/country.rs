use core::fmt;
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Country {
    SE,
}

impl Country {
    pub const fn all() -> &'static [Self] {
        &[Country::SE]
    }
}

impl Country {
    pub fn code(&self) -> &'static str {
        match self {
            Country::SE => "SE",
        }
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
