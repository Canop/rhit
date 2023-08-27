use {
    crate::ParseDateTimeError,
    std::fmt,
};

pub static MONTHS_3_LETTERS: &[&str] = &[
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
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ParseDateTimeError> {
        if day < 1 || day > 31 {
            return Err(ParseDateTimeError::InvalidDay(day));
        }
        if month < 1 || month > 12 {
            return Err(ParseDateTimeError::InvalidMonth(month));
        }
        Ok(Self { year, month, day })
    }
    /// parse the date part of a nginx datetime.
    ///
    /// a datetime in nginx is either in
    /// - "common log format", eg `10/Jan/2021:10:27:01 +0000`
    /// - ISO 8601, eg `1977-04-22T01:00:00-05:00`
    pub fn from_nginx(s: &str) -> Result<Self, ParseDateTimeError> {
        if s.len()<11 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        if let Ok(year) = s[0..4].parse() {
            // let's go with ISO 8601
            let month = s[5..7].parse()?;
            let day = s[8..10].parse()?;
            Self::new(year, month, day)
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
            Self::new(year, month, day)
        }
    }
    /// parse a numeric date with optionally implicit parts
    /// The part separator is the '/'
    pub fn with_implicit(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, ParseDateTimeError> {
        let mut t = s.split('/');
        match (t.next(), t.next(), t.next()) {
            (Some(year), Some(month), Some(day)) => {
                Date::new(year.parse()?, month.parse()?, day.parse()?)
            }
            (Some(month), Some(day), None) => {
                if let Some(year) = default_year {
                    Date::new(year, month.parse()?, day.parse()?)
                } else {
                    Err(ParseDateTimeError::AmbiguousDate(s.to_owned()))
                }
            }
            (Some(day), None, None) => {
                if let (Some(year), Some(month)) = (default_year, default_month) {
                    Date::new(year, month, day.parse()?)
                } else {
                    Err(ParseDateTimeError::AmbiguousDate(s.to_owned()))
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
    fn parse_nginx_date_common_log_format() {
        assert_eq!(
            Date::from_nginx("10/Jan/2021:10:27:01 +0000").unwrap(),
            Date::new(2021, 1, 10).unwrap(),
        );
    }
    #[test]
    fn parse_nginx_date_iso_8601() {
        assert_eq!(
            Date::from_nginx("1977-04-22T01:00:00-05:00").unwrap(),
            Date::new(1977, 4, 22).unwrap(),
        );
    }
}

pub fn unique_year_month(start_date: Date, end_date: Date) -> (Option<u16>, Option<u8>) {
    let y1 = start_date.year;
    let y2 = end_date.year;
    if y1 == y2 {
        let m1 = start_date.month;
        let m2 = end_date.month;
        if m1 == m2 {
            (Some(y1), Some(m1))
        } else {
            (Some(y1), None)
        }
    } else {
        (None, None)
    }
}
