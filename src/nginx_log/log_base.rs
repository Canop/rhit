pub use {
    crate::*,
    anyhow::*,
    chrono::{self, DateTime, Duration, FixedOffset, TimeZone},
    itertools::*,
    regex::Regex,
    std::{
        fs::File,
        io::{Read, BufRead, BufReader},
        path::{Path, PathBuf},
        str::FromStr,
    },
};

pub struct LogBase {
    pub lines: Vec<LogLine>,
}

impl LogBase {
    pub fn new(path: &Path) -> Result<Self> {
        let mut files = Vec::new();
        read_file(path.to_path_buf(), &mut files)?;
        println!("found {} files", files.len());
        if files.is_empty() {
            bail!("no log file found in {:?}", path);
        }
        files.sort_by_key(LogFile::start_time);
        let mut lines = Vec::new();
        for (a, b) in files.iter().tuple_windows() {
            let hole = b.start_time() - a.end_time();
            if hole < Duration::seconds(0) {
                bail!("inconsistent sequence");
            }
            if hole.num_minutes() > 60 {
                println!("hole of {} minutes", hole.num_minutes());
            }
            if hole.num_hours() > 20 {
                bail!("hole in log: {} hours are missing", hole.num_hours());
            }
        }
        for mut file in files {
            //println!("log file {:?} starts at {}", &file.path, file.start_time());
            lines.append(&mut file.lines);
        }
        Ok(Self {
            lines,
        })
    }
    pub fn retain_paths_matching(&mut self, pattern: &Regex) {
        self.lines.retain(|ll| pattern.is_match(&ll.path));
    }
    pub fn start_time(&self) -> DateTime<FixedOffset> {
        self.lines[0].time_local
    }
    pub fn end_time(&self) -> DateTime<FixedOffset> {
        self.lines[self.lines.len()-1].time_local
    }
}

fn read_file(path: PathBuf, files: &mut Vec<LogFile>) -> Result<()> {
    if path.is_dir() {
        println!("reading dir {:?}", &path);
        for entry in path.read_dir()? {
            read_file(entry?.path(), files)?;
        }
    } else if LogFile::is_access_log_path(&path) {
        println!("reading log file {:?}", &path);
        files.push(LogFile::new(path)?);
    }
    Ok(())
}
