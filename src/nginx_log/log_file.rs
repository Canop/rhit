use {
    crate::*,
    anyhow::*,
    flate2::bufread::GzDecoder,
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
        let file = File::open(&path)?;
        if path.extension().and_then(|e| e.to_str()) == Some("gz") {
            let file = BufReader::new(file);
            LogFile::read(GzDecoder::new(file), path)
        } else {
            LogFile::read(file, path)
        }
    }
    fn read<R: Read>(file: R, path: PathBuf) -> Result<LogFile> {
        debug!("reading file {:?}", &path);
        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut line = String::new();
        let mut errors = 0;
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
                    // we only log the first error
                    match errors {
                        0 => warn!("{} in {}", e, line),
                        1 => warn!("not logging other errors in this file"),
                        _ => {}
                    }
                    errors += 1;
                }
            }
        }
        if errors > 0 {
            warn!("{} errors in {:?}", errors, &path);
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
                name.contains("access.log")
            })
    }
    pub fn start_time(&self) -> Date {
        self.lines[0].date
    }
    pub fn end_time(&self) -> Date {
        self.lines[self.lines.len() - 1].date
    }
}

