use {
    crate::*,
};

#[derive(Debug, Clone, Copy)]
pub enum DateFilter {
    After(Date),
    Before(Date),
    Not(Date),
    Precise(Date),
    Range(Date, Date),
}

impl DateFilter {
    pub fn from_arg(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, DateParseError> {
        if let Some(s) = s.strip_prefix('>') {
            return Ok(Self::After(
                Date::with_implicit(s, default_year, default_month)?
            ));
        }
        if let Some(s) = s.strip_prefix('<') {
            return Ok(Self::Before(
                Date::with_implicit(s, default_year, default_month)?
            ));
        }
        if let Some(s) = s.strip_prefix('!') {
            return Ok(Self::Not(
                Date::with_implicit(s, default_year, default_month)?
            ));
        }
        let mut tokens = s.split('-');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(a), Some(b)) => Self::Range(
                Date::with_implicit(a, default_year, default_month)?,
                Date::with_implicit(b, default_year, default_month)?,
            ),
            (Some(a), None) => {
                if regex!(r#"^(\d{4})$"#).is_match(a) {
                    let year = a.parse()?;
                    Self::Range(
                        Date::new(year, 1, 1)?,
                        Date::new(year, 12, 31)?,
                    )
                } else if let Some(captures) = regex!(r#"^(\d{4})/(\d\d)$"#).captures(a) {
                    let year = captures[1].parse()?;
                    let month = captures[2].parse()?;
                    Self::Range(
                        Date::new(year, month, 1)?,
                        Date::new(year, month, 31)?, // we don't care whether it exists
                    )
                } else {
                    Self::Precise(
                        Date::with_implicit(a, default_year, default_month)?,
                    )
                }
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        })
    }
    pub fn contains(self, candidate: Date) -> bool {
        match self {
            Self::After(date) => date < candidate,
            Self::Before(date) => date > candidate,
            Self::Not(date) => date != candidate,
            Self::Precise(date) => date == candidate,
            Self::Range(a, b) => a <= candidate && candidate <= b,
        }
    }
}

#[cfg(test)]
mod date_filter_tests {

    use super::*;

    #[test]
    fn test_date_filter_fully_defined_range() {
        let df = DateFilter::from_arg("2021/01/03-2021/02/15", None, None).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 01).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), false);
    }

    #[test]
    fn test_date_filter_precise_date() {
        let df = DateFilter::from_arg("2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), false);
    }

    #[test]
    fn test_date_filter_not_date() {
        let df = DateFilter::from_arg("!2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), true);
    }

    #[test]
    fn test_date_filter_after_date_implicit_year() {
        let df = DateFilter::from_arg(">02/15", Some(2021), None).unwrap();
        assert_eq!(df.contains(Date::new(2020, 11, 12).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), true);
    }

    #[test]
    fn test_date_filter_after_date() {
        let df = DateFilter::from_arg(">2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), true);
    }

    #[test]
    fn test_date_filter_before_date() {
        let df = DateFilter::from_arg("<2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), false);
    }

    #[test]
    fn test_date_filter_default_year() {
        let df = DateFilter::from_arg("02/15", Some(2021), Some(02)).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), false);
    }

    #[test]
    fn test_date_filter_default_month_year() {
        let df = DateFilter::from_arg("15", Some(2021), Some(02)).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 16).unwrap()), false);
    }

    #[test]
    fn test_date_filter_month() {
        let df = DateFilter::from_arg("2021/02", Some(2021), Some(02)).unwrap();
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 03, 01).unwrap()), false);
    }

    #[test]
    fn test_date_filter_year() {
        let df = DateFilter::from_arg("2021", Some(2021), Some(02)).unwrap();
        assert_eq!(df.contains(Date::new(2020, 12, 28).unwrap()), false);
        assert_eq!(df.contains(Date::new(2021, 01, 28).unwrap()), true);
        assert_eq!(df.contains(Date::new(2021, 02, 15).unwrap()), true);
        assert_eq!(df.contains(Date::new(2022, 03, 01).unwrap()), false);
    }
}
