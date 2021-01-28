use {
    crate::*,
    anyhow::*,
    chrono::{self, DateTime, FixedOffset},
    std::{
        fs::File,
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
        str::FromStr,
    },
};

pub struct LogFile {
    pub path: PathBuf,
    // by construction, lines is guaranteed not empty
    pub lines: Vec<LogLine>,
}

impl LogFile {
    pub fn new(path: PathBuf) -> Result<LogFile> {
        let mut reader = BufReader::new(File::open(&path)?);
        let mut lines = Vec::new();
        let mut line = String::new();
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break; // EOF
            }
            match LogLine::from_str(&line) {
                Ok(log_line) => {
                    lines.push(log_line);
                }
                Err(e) => {
                    eprintln!("{} in {}", e, line);
                }
            }
        }
        if lines.is_empty() {
            bail!("empty log file");
        }
        Ok(Self {
            path,
            lines,
        })
    }
    pub fn is_access_log_path(path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |name| {
                let mut tokens = name.split('.');
                tokens.next() == Some("access") && tokens.next() == Some("log")
            })
    }
    pub fn start_time(&self) -> DateTime<FixedOffset> {
        self.lines[0].time_local
    }
    pub fn end_time(&self) -> DateTime<FixedOffset> {
        self.lines[self.lines.len()-1].time_local
    }
}

