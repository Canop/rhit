use {
    std::{
        fmt,
        num::ParseIntError,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum DateParseError {

    #[error("unexpected end")]
    UnexpectedEnd,

    #[error("invalid day {0:?}")]
    InvalidDay(u8),

    #[error("date is ambiguous in context {0:?}")]
    AmbiguousDate(String),

    #[error("invalid month {0:?}")]
    InvalidMonth(u8),

    #[error("unrecognized month {0:?}")]
    UnrecognizedMonth(String),

    #[error("expected int")]
    IntExpected(#[from] ParseIntError),
}

static MONTHS_3_LETTERS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];

/// a not precise date, only valid in the context
/// of the local set of log files.
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
/// As nginx didn't exist before JC, a u16 is good enough
/// for the year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8, // in [1,12]
    pub day: u8,   // in [1,31]
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, DateParseError> {
        if day < 1 || day > 31 {
            return Err(DateParseError::InvalidDay(day));
        }
        if month < 1 || month > 12 {
            return Err(DateParseError::InvalidMonth(month));
        }
        Ok(Self { year, month, day })
    }
    /// a datetime in nginx logs looks like this: `10/Jan/2021:10:27:01 +0000`
    pub fn from_nginx(s: &str) -> Result<Self, DateParseError> {
        if s.len()<11 {
            return Err(DateParseError::UnexpectedEnd);
        }
        let day = s[0..2].parse()?;
        let year = s[7..11].parse()?;
        let month = &s[3..6];
        let month = MONTHS_3_LETTERS
            .iter()
            .position(|&m| m == month)
            .ok_or_else(|| DateParseError::UnrecognizedMonth(s.to_string()))?;
        let month = (month + 1) as u8;
        Self::new(year, month, day)
    }
    /// parse a numeric date with optionally implicit parts
    /// The part separator is the '/'
    pub fn with_implicit(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, DateParseError> {
        let mut t = s.split('/');
        match (t.next(), t.next(), t.next()) {
            (Some(year), Some(month), Some(day)) => {
                Date::new(year.parse()?, month.parse()?, day.parse()?)
            }
            (Some(month), Some(day), None) => {
                if let Some(year) = default_year {
                    Date::new(year, month.parse()?, day.parse()?)
                } else {
                    Err(DateParseError::AmbiguousDate(s.to_string()))
                }
            }
            (Some(day), None, None) => {
                if let (Some(year), Some(month)) = (default_year, default_month) {
                    Date::new(year, month, day.parse()?)
                } else {
                    Err(DateParseError::AmbiguousDate(s.to_string()))
                }
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{:0>2}/{:0>2}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod date_parsing_tests {

    use super::*;

    #[test]
    fn parse_nginx_date() {
        assert_eq!(
            Date::from_nginx("10/Jan/2021:10:27:01 +0000").unwrap(),
            Date::new(2021, 1, 10).unwrap(),
        );
    }
}

