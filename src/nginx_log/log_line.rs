use {
    crate::*,
    std::{
        num::ParseIntError,
        str::FromStr,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ParseLogError {
    #[error("invalid log line {0:?}")]
    InvalidLogLine(String),
    #[error("character not found {0:?}")]
    CharNotFound(char),
    #[error("date parse error")]
    InvalidDateTime(#[from] ParseDateTimeError),
    #[error("expected int")]
    IntExpected(#[from] ParseIntError),
}

/// A line in the access log, describing a hit.
// perf note: parsing the remote adress as IP is costly
// (app is about 3% faster if I replace this field with a string)
#[derive(Debug)]
pub struct LogLine {
    pub remote_addr: Box<str>,
    pub date_time: DateTime,
    pub date_idx: usize,
    pub method: Method,
    pub path: Box<str>,
    pub status: u16,
    pub bytes_sent: u64,
    pub referer: Box<str>,
}

impl DateIndexed for LogLine {
    fn date_idx(&self) -> usize {
        self.date_idx
    }
    fn bytes(&self) -> u64 {
        self.bytes_sent
    }
}
impl DateIndexed for &LogLine {
    fn date_idx(&self) -> usize {
        self.date_idx
    }
    fn bytes(&self) -> u64 {
        self.bytes_sent
    }
}

impl LogLine {
    pub fn is_resource(&self) -> bool {
        let s = &self.path;
        s.ends_with(".png")
            || s.ends_with(".css")
            || s.ends_with(".svg")
            || s.ends_with(".jpg")
            || s.ends_with(".jpeg")
            || s.ends_with(".gif")
            || s.ends_with(".ico")
            || s.ends_with(".js")
            || s.ends_with(".woff2")
            || s.ends_with(".webp")
        // verified to be much much slower:
        // lazy_regex::regex_is_match!(
        //     "\\.(png|css|svg|jpe?g|gif|ico|js|woff2|webp)$",
        //     &self.path,
        // )
    }
    pub fn date(&self) -> Date {
        self.date_time.date
    }
    pub fn time(&self) -> Time {
        self.date_time.time
    }
}

impl FromStr for LogLine {
    type Err = ParseLogError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranger = Ranger::new(s);
        let remote_addr = ranger.until(' ')?.into();
        let date_time = DateTime::from_nginx(ranger.between('[', ']')?)?;
        let mut request = ranger.between('"', '"')?.split(' ');
        let (method, path) = match (request.next(), request.next()) {
            (Some(method), Some(path)) => (Method::from(method), path),
            (Some(path), None) => (Method::None, path),
            _ => unreachable!(),
        };
        let path = path.split('?').next().unwrap().into();
        let status = ranger.between(' ', ' ')?.parse()?;
        let bytes_sent = ranger.between(' ', ' ')?.parse()?;
        let referer = ranger.between('"', '"')?.into();
        Ok(LogLine {
            remote_addr,
            date_time,
            date_idx: 0,
            method,
            path,
            status,
            bytes_sent,
            referer,
        })
    }
}

#[cfg(test)]
mod log_line_parsing_tests {

    use super::*;

    static SIO_PULL_LINE: &str = r#"10.232.28.160 - - [22/Jan/2021:02:49:30 +0000] "GET /socket.io/?EIO=3&transport=polling&t=NSd_nu- HTTP/1.1" 200 99 "https://miaou.dystroy.org/3" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36""#;
    #[test]
    fn parse_sio_line() {
        let ll = LogLine::from_str(SIO_PULL_LINE).unwrap();
        assert_eq!(&*ll.remote_addr, "10.232.28.160");
        assert_eq!(ll.method, Method::Get);
        assert_eq!(&*ll.path, "/socket.io/");
        assert_eq!(ll.status, 200);
        assert_eq!(ll.bytes_sent, 99);
        assert_eq!(&*ll.referer, "https://miaou.dystroy.org/3");
    }

    static NO_VERB_LINE: &str = r#"119.142.145.250 - - [10/Jan/2021:10:27:01 +0000] "\x16\x03\x01\x00u\x01\x00\x00q\x03\x039a\xDF\xCA\x90\xB1\xB4\xC2SB\x96\xF0\xB7\x96CJD\xE1\xBF\x0E\xE1Y\xA2\x87v\x1D\xED\xBDo\x05A\x9D\x00\x00\x1A\xC0/\xC0+\xC0\x11\xC0\x07\xC0\x13\xC0\x09\xC0\x14\xC0" 400 173 "-" "-""#;
    #[test]
    fn parse_no_method_line() {
        let ll = LogLine::from_str(NO_VERB_LINE).unwrap();
        assert_eq!(ll.method, Method::None);
        assert_eq!(ll.status, 400);
        assert_eq!(ll.bytes_sent, 173);
    }


    static ISSUE_3_LINE: &str = r#"0.0.0.0 - - [2021-03-03T09:08:37+08:00] "GET /zhly/assets/guide/audit-opinion.png HTTP/1.1" 200 3911 "http://0.0.0.0:8091/zhly/" "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4427.5 Safari/537.36" "-""#;
    #[test]
    fn parse_issue_3_line() {
        let ll = LogLine::from_str(ISSUE_3_LINE).unwrap();
        assert_eq!(&*ll.remote_addr, "0.0.0.0");
        assert_eq!(ll.method, Method::Get);
        assert_eq!(ll.status, 200);
    }
}

