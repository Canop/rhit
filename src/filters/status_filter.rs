use {
    smallvec::*,
    std::{
        num::ParseIntError,
        str::FromStr,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ParseStatusFilterError {
    #[error("invalid int")]
    ParseInt(#[from] ParseIntError),
}

/// A filter for status, allowing classes, ranges and exclusions
/// Examples:
///  `4xx`
///  `4xx,503`
///  `402-417,503`
///  `4xx,!404`
#[derive(Debug, Clone)]
pub struct StatusFilter {
    include: SmallVec<[(u16, u16); 4]>,
    exclude: SmallVec<[(u16, u16); 4]>,
}

fn ranges_contains(ranges: &[(u16, u16)], status: u16) -> bool {
    for range in ranges {
        if range.0 <= status && status <= range.1 {
            return true;
        }
    }
    false
}

fn parse_range(s: &str) -> Result<(u16, u16), ParseStatusFilterError> {
    Ok(match s {
        "2xx" => (200, 299),
        "3xx" => (300, 399),
        "4xx" => (400, 499),
        "5xx" => (500, 599),
        s if s.contains('-') => {
            let mut tokens = s.split('-');
            (
                tokens.next().unwrap().parse()?,
                tokens.next().unwrap().parse()?,
            )
        }
        s => {
            let v = s.parse()?;
            (v, v)
        }
    })
}

impl StatusFilter {
    pub fn accepts(&self, status: u16) -> bool {
        if ranges_contains(&self.exclude, status) {
            false
        } else if self.include.is_empty() {
            true
        } else {
            ranges_contains(&self.include, status)
        }
    }
}

impl FromStr for StatusFilter {
    type Err= ParseStatusFilterError;
    fn from_str(value: &str) -> Result<Self, ParseStatusFilterError> {
        let mut include = SmallVec::new();
        let mut exclude = SmallVec::new();
        for s in value.split(',') {
            let s = s.trim();
            if let Some(s) = s.strip_prefix('!') {
                let s = s.trim();
                exclude.push(parse_range(s)?);
            } else {
                include.push(parse_range(s)?);
            }
        }
        Ok(Self { include, exclude })
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod status_filter_tests {

    use super::*;

    #[test]
    fn test_status_filter() {
        let sf = StatusFilter::from_str("400").unwrap();
        assert_eq!(sf.accepts(400), true);
        assert_eq!(sf.accepts(401), false);
        let sf = StatusFilter::from_str("2xx,405-512").unwrap();
        assert_eq!(sf.accepts(200), true);
        assert_eq!(sf.accepts(299), true);
        assert_eq!(sf.accepts(300), false);
        assert_eq!(sf.accepts(400), false);
        assert_eq!(sf.accepts(405), true);
        assert_eq!(sf.accepts(512), true);
        assert_eq!(sf.accepts(513), false);
        let sf = StatusFilter::from_str("4xx, ! 404").unwrap();
        assert_eq!(sf.accepts(200), false);
        assert_eq!(sf.accepts(400), true);
        assert_eq!(sf.accepts(404), false);
        assert_eq!(sf.accepts(421), true);
    }
}

