use chrono::NaiveDate;
use serde::{Serialize, Serializer, ser::SerializeSeq};

pub(super) const fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

pub(super) fn skip_nones<S, T>(items: &[Option<T>; 2], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let filtered: Vec<_> = items.iter().filter_map(|x| x.as_ref()).collect();
    let mut seq = serializer.serialize_seq(Some(filtered.len()))?;
    for item in filtered {
        seq.serialize_element(item)?;
    }
    seq.end()
}
