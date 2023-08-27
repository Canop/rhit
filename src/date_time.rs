use {
    crate::{
        Date,
        MONTHS_3_LETTERS,
        Time,
    },
    std::{
        fmt,
        num::ParseIntError,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ParseDateTimeError {

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

    #[error("invalid hour {0:?}")]
    InvalidHour(u8),

    #[error("invalid minute {0:?}")]
    InvalidMinute(u8),

    #[error("invalid second {0:?}")]
    InvalidSecond(u8),

    #[error("expected int")]
    IntExpected(#[from] ParseIntError),

    #[error("expected int")]
    IntExpectedInternal,
}


/// a date with time.
///
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub fn new(
        year: u16, // in [0-4000]
        month: u8, // in [1-12]
        day: u8, // in [1-31]
        hour: u8, // in [0-23]
        minute: u8,
        second: u8,
    ) -> Result<Self, ParseDateTimeError> {
        Ok(Self {
            date: Date::new(year, month, day)?,
            time: Time::new(hour, minute, second)?,
        })
    }
    /// parse the date_time part of a nginx log line
    ///
    /// a datetime in nginx is either in
    /// - "common log format", eg `10/Jan/2021:10:27:01 +0000`
    /// - ISO 8601, eg `1977-04-22T01:00:00-05:00`
    pub fn from_nginx(s: &str) -> Result<Self, ParseDateTimeError> {
        if s.len()<20 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        if let Ok(year) = s[0..4].parse() {
            // let's go with ISO 8601
            let month = s[5..7].parse()?;
            let day = s[8..10].parse()?;
            let hour = s[11..13].parse()?;
            let minute = s[14..16].parse()?;
            let second = s[17..19].parse()?;
            Self::new(year, month, day, hour, minute, second)
        } else {
            // maybe common log format ?
            let day = s[0..2].parse()?;
            let month = &s[3..6];
            let month = MONTHS_3_LETTERS
                .iter()
                .position(|&m| m == month)
                .ok_or_else(|| ParseDateTimeError::UnrecognizedMonth(s.to_owned()))?;
            let month = (month + 1) as u8;
            let year = s[7..11].parse()?;
            let hour = s[12..14].parse()?;
            let minute = s[15..17].parse()?;
            let second = s[18..20].parse()?;
            Self::new(year, month, day, hour, minute, second)
        }
    }
    pub fn round_up(date: Date, time: Option<Time>) -> Self {
        Self {
            date,
            time: time.unwrap_or(Time { hour:23, minute:59, second:59}),
        }
    }
    pub fn round_down(date: Date, time: Option<Time>) -> Self {
        Self {
            date,
            time: time.unwrap_or(Time { hour:0, minute:0, second:0}),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{:0>2}/{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        )
    }
}

#[cfg(test)]
mod date_time_parsing_tests {

    use super::*;

    #[test]
    fn parse_nginx_date_common_log_format() {
        assert_eq!(
            DateTime::from_nginx("10/Jan/2021:11:27:02 +0000").unwrap(),
            DateTime::new(2021, 1, 10, 11, 27, 2).unwrap(),
        );
    }
    #[test]
    fn parse_nginx_date_iso_8601() {
        assert_eq!(
            DateTime::from_nginx("1977-04-22T12:51:23-05:00").unwrap(),
            DateTime::new(1977, 4, 22, 12, 51, 23).unwrap(),
        );
    }
}
