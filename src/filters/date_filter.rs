use {
    crate::*,
};

#[derive(Debug, Clone, Copy)]
pub enum DateFilter {
    Precise(Date),
    Range(Date, Date),
}

impl DateFilter {
    pub fn from_arg(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, DateParseError> {
        let mut tokens = s.split('-');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(a), Some(b)) => Self::Range(
                Date::with_implicit(a, default_year, default_month)?,
                Date::with_implicit(b, default_year, default_month)?,
            ),
            (Some(a), None) => Self::Precise(
                Date::with_implicit(a, default_year, default_month)?,
            ),
            _ => unsafe { std::hint::unreachable_unchecked() },
        })
    }
    pub fn contains(self, candidate: Date) -> bool {
        match self {
            Self::Precise(date) => date == candidate,
            Self::Range(a, b) => a <= candidate && candidate <= b,
        }
    }
}
