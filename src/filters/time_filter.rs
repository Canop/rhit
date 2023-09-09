use {
    crate::*,
    lazy_regex::*,
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ParseTimeFilterError {
    #[error("invalid time filter format")]
    InvalidFormat,
    #[error("Expected time")]
    TimeParse(#[from] ParseDateTimeError),
}

#[derive(Debug, Clone, Copy)]
pub enum TimeFilter {
    After(Time),
    Before(Time),
    Range(Time, Time),
}

impl TimeFilter {
    pub fn contains(self, canditime: Time) -> bool {
        match self {
            Self::After(time) => time < canditime,
            Self::Before(time) => time > canditime,
            Self::Range(a, b) => {
                if a <= b {
                    a <= canditime && canditime <= b
                } else {
                    // you can ask for hits between 22h and 4h
                    canditime >= a || canditime <= b
                }
            }
        }
    }
}

impl FromStr for TimeFilter {
    type Err= ParseTimeFilterError;
    fn from_str(s: &str) -> Result<Self, ParseTimeFilterError> {
        if let Some(s) = s.strip_prefix('>') {
            let s = s.trim();
            return Ok(Self::After(
                s.parse()?
            ));
        }
        if let Some(s) = s.strip_prefix('<') {
            let s = s.trim();
            return Ok(Self::Before(
                s.parse()?
            ));
        }
        if let Some((_, min, max)) = regex_captures!(r"^\s*([^-]+)\s*-\s*([^-]+)\s*$", s) {
            return Ok(Self::Range( min.parse()?, max.parse()?));
        }
        Err(ParseTimeFilterError::InvalidFormat)
    }
}
