use {
    crate::ParseDateTimeError,
    std::{fmt, str::FromStr},
};

/// a time, only valid in the context of the local set of log files.
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub hour: u8, // in [0,23]
    pub minute: u8, // in [0,59]
    pub second: u8,   // in [0,59]
}
impl Time {
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, ParseDateTimeError> {
        if second > 59 {
            return Err(ParseDateTimeError::InvalidSecond(second));
        }
        if minute > 59 {
            return Err(ParseDateTimeError::InvalidMinute(minute));
        }
        if hour > 23 {
            return Err(ParseDateTimeError::InvalidHour(hour));
        }
        Ok(Self { hour, minute, second })
    }
}

impl FromStr for Time {
    type Err = ParseDateTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()<2 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        let hour = s[0..2].parse()?;
        let mut minute = 0;
        let mut second = 0;
        if s.len()>4 {
            minute = s[3..5].parse()?;
            if s.len()>6 {
                second = s[6..7].parse()?;
            }
        }
        Time::new(hour, minute, second)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
}
