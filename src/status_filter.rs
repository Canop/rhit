use {
    std::{
        num::ParseIntError,
        str::FromStr,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum StatusFilterParseError {
    #[error("invalid int")]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
pub struct StatusFilter {
    ranges: Vec<(u16, u16)>,
}

impl StatusFilter {
    pub fn contains(&self, status: u16) -> bool {
        for range in &self.ranges {
            if range.0 <= status && status <= range.1 {
                return true;
            }
        }
        false
    }
}

impl FromStr for StatusFilter {
    type Err= StatusFilterParseError;
    fn from_str(value: &str) -> Result<Self, StatusFilterParseError> {
        let ranges: Result<Vec<(u16, u16)>, StatusFilterParseError> = value
            .split(',')
            .map(|s| {
                Ok(match s {
                    "2xx" => (200, 299),
                    "3xx" => (300, 399),
                    "4xx" => (400, 499),
                    "5xx" => (500, 599),
                    s if s.contains('-') => {
                        let mut tokens = s.split('-');
                        (
                            tokens.next().unwrap().parse()?,
                            tokens.next().unwrap().parse()?
                        )
                    }
                    s => {
                        let v = s.parse()?;
                        (v, v)
                    }
                })
            })
            .collect();
        let ranges = ranges?;
        Ok(Self { ranges })
    }
}

#[cfg(test)]
mod status_filter_tests {

    use super::*;

    #[test]
    fn test_status_filter() {
        let sf = StatusFilter::from_str("400").unwrap();
        assert_eq!(sf.contains(400), true);
        assert_eq!(sf.contains(401), false);
        let sf = StatusFilter::from_str("2xx,405-512").unwrap();
        assert_eq!(sf.contains(200), true);
        assert_eq!(sf.contains(299), true);
        assert_eq!(sf.contains(300), false);
        assert_eq!(sf.contains(400), false);
        assert_eq!(sf.contains(405), true);
        assert_eq!(sf.contains(512), true);
        assert_eq!(sf.contains(513), false);
    }
}

