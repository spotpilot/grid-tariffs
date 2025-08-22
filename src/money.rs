use std::{
    fmt::{Display, Formatter},
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum MoneyError {
    #[error("Invalid money")]
    InvalidMoney,
    #[error("Money sub-unit is more than 99")]
    SubunitOutOfRange,
}

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money {
    pub const ZERO: Self = Self(0);
    const GAIN: i64 = 100000;
    const GAIN_F64: f64 = 100000.;

    pub const fn new(int: i64, fract: u8) -> Self {
        let base = int * Self::GAIN;
        let sub = fract as i64 * Self::GAIN / 100;
        Self(if base.is_negative() {
            base - sub
        } else {
            base + sub
        })
    }

    /// From a subunit amount (öre, cents etc)
    /// E.g. Money::new_subunit(8.86) -> Money(8860)
    pub const fn new_subunit(value: f64) -> Self {
        Self((value * Self::GAIN_F64 / 100.) as i64)
    }

    pub const fn display(&self, currency: Currency) -> MoneyDisplay {
        MoneyDisplay(*self, currency)
    }

    pub fn inner(&self) -> i64 {
        self.0
    }

    pub fn from_inner(inner: i64) -> Self {
        Self(inner)
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / Self::GAIN_F64
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{:.2}", self.to_f64()).fmt(f)
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((int, fract)) = s.split_once('.') {
            let int: i64 = int.parse().map_err(|_| MoneyError::InvalidMoney)?;
            let fract: u8 = fract.parse().map_err(|_| MoneyError::SubunitOutOfRange)?;
            Ok(Self::new(int, fract))
        } else {
            let int: i64 = s.parse().map_err(|_| MoneyError::InvalidMoney)?;
            Ok(Self::new(int, 0))
        }
    }
}

impl From<f64> for Money {
    fn from(value: f64) -> Self {
        Self((value * Self::GAIN_F64) as i64)
    }
}

impl From<i32> for Money {
    fn from(value: i32) -> Self {
        Self::new(value.into(), 0)
    }
}

impl From<(i32, u8)> for Money {
    fn from((int, fract): (i32, u8)) -> Self {
        Self::new(int.into(), fract)
    }
}

impl From<Money> for f64 {
    fn from(value: Money) -> Self {
        value.to_f64()
    }
}

impl Mul<f64> for Money {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.to_f64() * rhs)
    }
}

impl Div<f64> for Money {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from(self.to_f64() / rhs)
    }
}

impl Div<i64> for Money {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl<'a> Sum<&'a Money> for Money {
    fn sum<I: Iterator<Item = &'a Money>>(iter: I) -> Self {
        iter.map(|m| m.to_owned()).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MoneyDisplay(pub Money, pub Currency);

impl std::fmt::Display for MoneyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.amount(), self.currency().sign()))
    }
}

impl MoneyDisplay {
    pub const fn money(&self) -> Money {
        self.0
    }
    pub const fn currency(&self) -> Currency {
        self.1
    }

    pub fn subunit_display(&self) -> String {
        format!(
            "{} {}",
            self.subunit_amount(),
            self.currency().subunit_sign()
        )
    }

    pub fn amount(&self) -> String {
        format!("{:.2}", self.money().to_f64())
    }

    pub fn subunit_amount(&self) -> String {
        format!("{:.0}", self.money().to_f64() * 100.)
    }
}

#[cfg(test)]
mod tests {
    use super::{Currency, Money};

    #[test]
    fn new_money_positive() {
        assert_eq!(Money::new(2, 14), Money(214000));
    }

    #[test]
    fn new_money_small() {
        assert_eq!(Money::new(0, 4), Money(4000));
    }

    #[test]
    fn new_money_negative() {
        assert_eq!(Money::new(-2, 14), Money(-214000));
    }

    #[test]
    fn from_negative_f64() {
        assert_eq!(Money::from(-2.14), Money(-214000));
    }

    #[test]
    fn from_small_f64() {
        assert_eq!(Money::from(0.04732), Money(4732));
    }

    #[test]
    fn new_subunit() {
        assert_eq!(Money::new_subunit(14.), Money(14_000));
        assert_eq!(Money::new_subunit(8.86), Money(8_860));
    }

    #[test]
    fn new_subunit_and_new_equal() {
        assert_eq!(Money::new_subunit(14.), Money::new(0, 14));
    }

    #[test]
    fn display_impl() {
        assert_eq!(Money(9000).to_string(), "0.09");
        assert_eq!(Money(199999).to_string(), "2.00");
        assert_eq!(Money(20_999_999).to_string(), "210.00");
    }

    #[test]
    fn display() {
        let m = Money::from((1, 42)).display(Currency::SEK);
        assert_eq!(m.to_string(), "1.42 kr");
    }

    #[test]
    fn display_small() {
        let m = Money::from((0, 9)).display(Currency::SEK);
        assert_eq!(m.to_string(), "0.09 kr");
    }

    #[test]
    fn subunit_display() {
        let m = Money::from((1, 42)).display(Currency::SEK);
        assert_eq!(m.subunit_display(), "142 öre");
    }

    #[test]
    fn amount_display() {
        let m = Money::from((1, 42)).display(Currency::SEK);
        assert_eq!(m.amount(), "1.42");
    }

    #[test]
    fn amount_display_small() {
        let m = Money(7765).display(Currency::SEK);
        assert_eq!(m.amount(), "0.08");
    }

    #[test]
    fn amount_display_large() {
        let m = Money::from((123456789, 99)).display(Currency::SEK);
        assert_eq!(m.amount(), "123456789.99");
    }

    #[test]
    fn subunit_amount_display() {
        let m = Money::from((1, 42)).display(Currency::SEK);
        assert_eq!(m.subunit_amount(), "142");
    }

    #[test]
    fn add_money() {
        assert_eq!(Money(1437), Money(100) + Money(1337));
    }

    #[test]
    fn mul_money_f64() {
        assert_eq!(Money(145) * 10f64, Money(1450));
    }

    #[test]
    fn into_f64() {
        let val: f64 = Money(145000).into();
        assert_eq!(val, 1.45);
    }

    #[test]
    fn sum_money_references() {
        assert_eq!([Money(100), Money(1237)].iter().sum::<Money>(), Money(1337));
    }

    #[test]
    fn sum_money_negative() {
        assert_eq!(
            [Money(-40000), Money(-3000), Money(-7500)]
                .iter()
                .sum::<Money>(),
            Money(-50500)
        );
    }

    #[test]
    fn sum_money_mixed() {
        assert_eq!(
            [Money(-40000), Money(5000), Money(8000)]
                .iter()
                .sum::<Money>(),
            Money(-27000)
        );
    }
}
