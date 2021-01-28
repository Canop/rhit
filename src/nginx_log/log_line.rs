use {
    crate::Ranger,
    chrono::{self, DateTime, FixedOffset},
    std::{
        net::{
            IpAddr,
            AddrParseError,
        },
        num::ParseIntError,
        str::FromStr,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum LogParseError {
    #[error("invalid remote addr")]
    InvalidRemoteAddr(#[from] AddrParseError),
    #[error("invalid log line {0:?}")]
    InvalidLogLine(String),
    #[error("character not found {0:?}")]
    CharNotFound(char),
    #[error("chrono parse error")]
    InvalidDate(#[from] chrono::ParseError),
    #[error("expected int")]
    IntExpected(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verb {
    Get,
    Post,
    None, // bad request
    Other,
}

impl From<&str> for Verb {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "" => Self::None,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogLine {
    pub remote_addr: IpAddr,
    pub time_local: DateTime<FixedOffset>,
    pub verb: Verb,
    pub path: String,
    pub status: u16,
    pub bytes_sent: u64,
}

impl LogLine {
    pub fn is_resource(&self) -> bool {
        let s = &self.path;
        s.ends_with(".png")
            || s.ends_with(".css")
            || s.ends_with(".svg")
            || s.ends_with(".jpg")
            || s.ends_with(".ico")
            || s.ends_with(".js")
            || s.ends_with(".woff2")
    }
    pub fn looks_like_download(&self) -> bool {
        let s = &self.path;
        s.contains("download")
            && !s.contains("index")
            && !s.ends_with(".php")
            && !self.is_resource()
    }
}

impl FromStr for LogLine {
    type Err = LogParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranger = Ranger::new(s);
        let remote_addr = IpAddr::from_str(ranger.until(' ')?)?;
        let time_local = DateTime::parse_from_str(
            ranger.between('[', ']')?,
            "%d/%b/%Y:%H:%M:%S %z",
        )?;
        let mut request = ranger.between('"', '"')?.split(' ');
        let (verb, path) = match (request.next(), request.next()) {
            (Some(verb), Some(path)) => (Verb::from(verb), path),
            (Some(path), None) => (Verb::None, path),
            _ => unreachable!(),
        };
        let path = path.split('?').next().unwrap().to_string();
        let status = ranger.between(' ', ' ')?.parse()?;
        let bytes_sent = ranger.between(' ', ' ')?.parse()?;
        Ok(LogLine {
            remote_addr,
            time_local,
            verb,
            path,
            status,
            bytes_sent,
        })
    }
}

#[cfg(test)]
mod log_line_parsing_tests {

    use {
        super::*,
        std::{
            net::{
                Ipv4Addr,
            },
        },
    };

    static SIO_PULL_LINE: &str = r#"109.23.28.160 - - [22/Jan/2021:02:49:30 +0000] "GET /socket.io/?EIO=3&transport=polling&t=NSd_nu- HTTP/1.1" 200 99 "https://miaou.dystroy.org/3" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36""#;
    #[test]
    fn parse_normal_line() -> anyhow::Result<()> {
        let ll = LogLine::from_str(SIO_PULL_LINE).unwrap();
        assert_eq!(ll.remote_addr, IpAddr::V4(Ipv4Addr::new(109, 23, 28, 160)));
        assert_eq!(ll.verb, Verb::Get);
        assert_eq!(ll.path, "/socket.io/");
        assert_eq!(ll.status, 200);
        assert_eq!(ll.bytes_sent, 99);
        Ok(())
    }

    static NO_VERB_LINE: &str = r#"139.162.145.250 - - [10/Jan/2021:10:27:01 +0000] "\x16\x03\x01\x00u\x01\x00\x00q\x03\x039a\xDF\xCA\x90\xB1\xB4\xC2SB\x96\xF0\xB7\x96CJD\xE1\xBF\x0E\xE1Y\xA2\x87v\x1D\xED\xBDo\x05A\x9D\x00\x00\x1A\xC0/\xC0+\xC0\x11\xC0\x07\xC0\x13\xC0\x09\xC0\x14\xC0" 400 173 "-" "-""#;

    #[test]
    fn parse_no_verb_line() -> anyhow::Result<()> {
        let ll = LogLine::from_str(NO_VERB_LINE).unwrap();
        assert_eq!(ll.verb, Verb::None);
        assert_eq!(ll.status, 400);
        assert_eq!(ll.bytes_sent, 173);
        Ok(())
    }
}


