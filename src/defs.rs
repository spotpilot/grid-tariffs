// A definition of hours
#[derive(Clone, Copy)]
pub(super) enum Hours {
    Irrelevant,
    FromToInclusive(u8, u8),
}

impl Hours {
    pub(super) const fn new(from: u8, to_inclusive: u8) -> Self {
        Self::FromToInclusive(from, to_inclusive)
    }

    /// Create new, but mark as not really used
    const fn irrelevant() -> Self {
        Self::Irrelevant
    }
}

// The supported main fuse sizes
#[derive(Clone, Copy)]
pub(super) struct MainFuseSizes {
    from: u16,
    to: u16,
}

impl MainFuseSizes {
    pub(super) const fn new_range(from: u16, to: u16) -> Self {
        Self { from, to }
    }
}

#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub(super) enum Months {
    Irrelevant,
    RangeInclusive(Month, Month),
}

impl Months {
    pub(super) const fn new(from: Month, to: Month) -> Self {
        Self::RangeInclusive(from, to)
    }

    const fn irrelevant() -> Self {
        Self::Irrelevant
    }
}
