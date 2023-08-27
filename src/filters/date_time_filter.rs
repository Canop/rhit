use {
    crate::*,
};

#[derive(Debug, Clone, Copy)]
pub enum DateTimeFilter {
    AfterDate(Date),
    AfterDateTime(DateTime),
    BeforeDate(Date),
    BeforeDateTime(DateTime),
    NotDate(Date),
    PreciseDate(Date),
    NotDateTime(DateTime),
    PreciseDateTime(DateTime),
    Range(DateTime, DateTime),
}

impl DateTimeFilter {
    pub fn new(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, ParseDateTimeError> {
        if let Some(s) = s.strip_prefix('>') {
            let (date, time) = parse_date_optional_time(s, default_year, default_month)?;
            return Ok(match time {
                Some(time) => Self::AfterDateTime(DateTime{date, time}),
                None => Self::AfterDate(date),
            });
        }
        if let Some(s) = s.strip_prefix('<') {
            let (date, time) = parse_date_optional_time(s, default_year, default_month)?;
            return Ok(match time {
                Some(time) => Self::BeforeDateTime(DateTime{date, time}),
                None => Self::BeforeDate(date),
            });
        }
        if let Some(s) = s.strip_prefix('!') {
            let (date, time) = parse_date_optional_time(s, default_year, default_month)?;
            match time {
                Some(time) => {
                    return Ok(Self::NotDateTime(
                        DateTime { date, time }
                    ));
                }
                None => {
                    return Ok(Self::NotDate(date));
                }
            }
        }
        let mut tokens = s.split('-');
        let a = tokens.next().unwrap(); // there's always a first token in a split
        if let Some(b) = tokens.next() {
            // two dates: a range
            let (da, ta) = parse_date_optional_time(a, default_year, default_month)?;
            let (db, tb) = parse_date_optional_time(b, default_year, default_month)?;
            Ok(Self::Range(
                DateTime::round_down(da, ta),
                DateTime::round_up(db, tb),
            ))
        } else {
            // one token, no modifier
            let toks: Vec<&str> = a
                .split(|c: char| !c.is_ascii_digit())
                .collect();
            let filter = match toks.len() {
                0 => {
                    return Err(ParseDateTimeError::UnexpectedEnd);
                }
                1 => {
                    if toks[0].len() == 4 {
                        // a year, user wants the whole year
                        let year = toks[0].parse()?;
                        Self::Range(
                            DateTime::new(year, 1, 1, 0, 0, 0)?,
                            DateTime::new(year, 12, 31, 23, 59, 59)?,
                        )
                    } else {
                        // only the day : the year and month must be provided
                        match (default_year, default_month) {
                            (Some(year), Some(month)) => {
                                Self::PreciseDate(Date::new(year, month, toks[0].parse()?)?)
                            }
                            _ => {
                                return Err(ParseDateTimeError::AmbiguousDate(s.to_owned()));
                            }
                        }
                    }
                }
                2 => {
                    if toks[0].len() == 4 { // year and month -> whole month
                        let year = toks[0].parse()?;
                        let month = toks[1].parse()?;
                        Self::Range(
                            DateTime::new(year, month, 1, 0, 0, 0)?,
                            DateTime::new(year, month, 31, 23, 59, 59)?,
                        )
                    } else { // month and day
                        // we need the year to be already known
                        match default_year {
                            Some(year) => {
                                Self::PreciseDate(Date::new(year, toks[0].parse()?, toks[1].parse()?)?)
                            }
                            None => {
                                return Err(ParseDateTimeError::AmbiguousDate(s.to_owned()));
                            }
                        }
                    }
                }
                3 => { // full date, no time
                    Self::PreciseDate(Date::new(toks[0].parse()?, toks[1].parse()?, toks[2].parse()?)?)
                }
                4 => { // date and hour
                    Self::PreciseDateTime(
                        DateTime::new(toks[0].parse()?, toks[1].parse()?, toks[2].parse()?, toks[3].parse()?, 0, 0)?,
                    )
                }
                5 => { // date, hour, and minute
                    Self::PreciseDateTime(
                        DateTime::new(toks[0].parse()?, toks[1].parse()?, toks[2].parse()?, toks[3].parse()?, toks[4].parse()?, 0)?,
                    )
                }
                _ => { // date and time
                    Self::PreciseDateTime(
                        DateTime::new(toks[0].parse()?, toks[1].parse()?, toks[2].parse()?, toks[3].parse()?, toks[4].parse()?, toks[5].parse()?)?,
                    )
                }
            };
            Ok(filter)
        }
    }
    #[inline(always)]
    pub fn contains(self, candidate: DateTime) -> bool {
        match self {
            Self::AfterDate(date) => date < candidate.date,
            Self::AfterDateTime(datetime) => datetime < candidate,
            Self::BeforeDate(date) => date > candidate.date,
            Self::BeforeDateTime(datetime) => datetime > candidate,
            Self::NotDate(date) => date != candidate.date,
            Self::NotDateTime(datetime) => datetime != candidate,
            Self::PreciseDate(date) => date == candidate.date,
            Self::PreciseDateTime(datetime) => datetime == candidate,
            Self::Range(a, b) => a <= candidate && candidate <= b,
        }
    }
    pub fn overlaps(self, candidate: Date) -> bool {
        match self {
            Self::AfterDate(date) => candidate > date,
            Self::AfterDateTime(datetime) => datetime.date < candidate,
            Self::BeforeDate(date) => date > candidate,
            Self::BeforeDateTime(datetime) => datetime.date > candidate,
            Self::NotDate(date) => date != candidate,
            Self::NotDateTime(_) => true,
            Self::PreciseDate(date) => date == candidate,
            Self::PreciseDateTime(datetime) => datetime.date == candidate,
            Self::Range(a, b) => a.date <= candidate && candidate <= b.date,
        }
    }
}

/// parse a numeric date with optionally implicit parts,
/// and optionally a time
fn parse_date_optional_time(
    s: &str,
    default_year: Option<u16>,
    default_month: Option<u8>,
) -> Result<(Date, Option<Time>), ParseDateTimeError> {
    let mut t = s.split(|c: char| !c.is_ascii_digit());
    let date = match (t.next(), t.next(), t.next()) {
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
    }?;
    let time = match t.next() {
        Some(hour) => {
            Some(Time::new(
                hour.parse()?,
                t.next().map(|m| m.parse()).transpose()?.unwrap_or(0),
                t.next().map(|s| s.parse()).transpose()?.unwrap_or(0),
            )?)
        }
        None => None,
    };
    Ok((date, time))
}


#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
#[allow(clippy::bool_assert_comparison)]
mod date_time_filter_tests {

    use super::*;

    macro_rules! date {
        ($year:literal, $month:literal, $day:literal) => {
            Date::new($year, $month, $day).unwrap()
        }
    }

    macro_rules! date_time {
        ($year:literal, $month:literal, $day:literal, $hour:literal, $minute:literal) => {
            DateTime::new($year, $month, $day, $hour, $minute, 0).unwrap()
        }
    }

    #[test]
    fn test_date_filter_fully_defined_range() {
        let df = DateTimeFilter::new("2021/01/03-2021/02/15", None, None).unwrap();
        assert_eq!(df.overlaps(date!(2021, 01, 28)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 01)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), false);
        assert_eq!(df.contains(date_time!(2021, 02, 16, 0, 1)), false);
        assert_eq!(df.contains(date_time!(2021, 01, 03, 0, 1)), true);
        assert_eq!(df.contains(date_time!(2021, 01, 03, 0, 0)), true);
        assert_eq!(df.contains(date_time!(2021, 01, 02, 23, 59)), false);
        assert_eq!(df.contains(date_time!(2022, 01, 04, 23, 59)), false);
    }

    #[test]
    fn test_date_filter_precise_date() {
        let df = DateTimeFilter::new("2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), false);
        assert_eq!(df.contains(date_time!(2021, 02, 15, 23, 59)), true);
        assert_eq!(df.contains(date_time!(2021, 02, 14, 23, 59)), false);
        assert_eq!(df.contains(date_time!(2021, 02, 16, 0, 0)), false);
    }

    #[test]
    fn test_date_filter_not_date() {
        let df = DateTimeFilter::new("!2021/02/15", Some(2021), None).unwrap();
        assert_eq!(df.overlaps(date!(2021, 01, 28)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), true);
        assert_eq!(df.contains(date_time!(2021, 02, 15, 2, 0)), false);
        assert_eq!(df.contains(date_time!(2021, 02, 16, 0, 0)), true);
    }

    #[test]
    fn test_date_filter_after_date_implicit_year() {
        let df = DateTimeFilter::new(">02/15", Some(2021), None).unwrap();
        dbg!(df);
        assert_eq!(df.overlaps(date!(2020, 11, 12)), false);
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), true);
    }

    #[test]
    fn test_date_filter_after_date() {
        let df = DateTimeFilter::new(">2021/02/15", Some(2021), None).unwrap();
        dbg!(df);
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), true);
    }

    #[test]
    fn test_date_filter_before_date() {
        let df = DateTimeFilter::new("<2021/02/15", Some(2021), None).unwrap();
        dbg!(df);
        assert_eq!(df.overlaps(date!(2021, 01, 28)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), false);
    }

    #[test]
    fn test_date_filter_default_year() {
        let df = DateTimeFilter::new("02/15", Some(2021), Some(02)).unwrap();
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), false);
    }

    #[test]
    fn test_date_filter_default_month_year() {
        let df = DateTimeFilter::new("15", Some(2021), Some(02)).unwrap();
        dbg!(df);
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 16)), false);
    }

    #[test]
    fn test_date_filter_month() {
        let df = DateTimeFilter::new("2021/02", Some(2021), Some(02)).unwrap();
        assert_eq!(df.overlaps(date!(2021, 01, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2021, 03, 01)), false);
    }

    #[test]
    fn test_date_filter_year() {
        let df = DateTimeFilter::new("2021", Some(2021), Some(02)).unwrap();
        assert_eq!(df.overlaps(date!(2020, 12, 28)), false);
        assert_eq!(df.overlaps(date!(2021, 01, 28)), true);
        assert_eq!(df.overlaps(date!(2021, 02, 15)), true);
        assert_eq!(df.overlaps(date!(2022, 03, 01)), false);
    }
}
