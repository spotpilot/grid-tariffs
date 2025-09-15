use serde::Serialize;

// A definition of hours (inclusive)
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct Hours(u8, u8);

impl Hours {
    pub(super) const fn new(from: u8, to_inclusive: u8) -> Self {
        Self(from, to_inclusive)
    }
}

// The supported main fuse sizes
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct MainFuseSizes {
    from: u16,
    to: u16,
}

impl MainFuseSizes {
    pub(super) const fn new_range(from: u16, to: u16) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(u8)]
pub(super) enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// An inclusive range of months
#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub(super) struct Months(Month, Month);

impl Months {
    pub(super) const fn new(from: Month, to: Month) -> Self {
        Self(from, to)
    }
}
