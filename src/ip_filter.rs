use {
    thiserror::Error,
    std::{
        net::{AddrParseError, IpAddr},
        str::FromStr,
    },
};

#[derive(Debug, Error)]
pub enum IpFilterParseError {

    #[error("invalid ip addr")]
    InvalidIpAddr(#[from] AddrParseError),

}

#[derive(Debug, Clone)]
pub struct IpFilter {
    negative: bool,
    addr: IpAddr,
}

impl IpFilter {
    pub fn new(mut pattern: &str) -> Result<Self, IpFilterParseError> {
        println!("patter: {:?}", pattern);
        let negative = pattern.starts_with('!');
        if negative {
            pattern = &pattern[1..];
        }
        let addr = IpAddr::from_str(pattern)?;
        Ok(Self { negative, addr })
    }
    pub fn accepts(&self, candidate: IpAddr) -> bool {
        if self.negative {
            self.addr != candidate
        } else {
            self.addr == candidate
        }
    }
}

#[cfg(test)]
mod ip_filter_tests {

    use super::*;

    #[test]
    fn test_v4() {
        let f = IpFilter::new("35.180.167.230").unwrap();
        assert_eq!(f.accepts(IpAddr::from_str("35.180.167.230").unwrap()), true);
        assert_eq!(f.accepts(IpAddr::from_str("123.123.123.123").unwrap()), false);
        let f = IpFilter::new("!35.180.167.230").unwrap();
        assert_eq!(f.accepts(IpAddr::from_str("35.180.167.230").unwrap()), false);
        assert_eq!(f.accepts(IpAddr::from_str("123.123.123.123").unwrap()), true);
    }
}
