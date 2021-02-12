pub use {
    crate::*,
    anyhow::*,
    crossterm::{
        self,
        cursor,
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        style::{style, Color, Print, PrintStyledContent},
        terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
        queue,
        QueueableCommand,
    },
    itertools::*,
    regex::Regex,
    std::{
        fs::File,
        io::{self, Read, BufRead, BufReader, Write},
        path::{Path, PathBuf},
        str::FromStr,
    },
    termimad::{MadSkin, ProgressBar},
};

pub struct LogBase {
    pub dates: Vec<Date>,
    pub lines: Vec<LogLine>,
}

impl LogBase {
    pub fn new(path: &Path) -> Result<Self> {
        let mut files = Vec::new();
        find_files(path.to_path_buf(), &mut files)?;
        let mut log_files = read_files(files)?;
        execute!(io::stderr(), Clear(ClearType::CurrentLine))?;
        if log_files.is_empty() {
            bail!("no log file found in {:?}", path);
        } else {
            eprintln!("I've read {} files in {:?}", log_files.len(), path);
        }
        log_files.sort_by_key(LogFile::start_time);
        let mut lines = Vec::new();
        for mut file in log_files {
            lines.append(&mut file.lines);
        }
        let mut dates = Vec::new();
        let mut cur_date: Option<Date> = None;
        for i in 0..lines.len() {
            if let Some(date) = &mut cur_date {
                if lines[i].date == *date {
                    lines[i].date_idx = dates.len();
                    continue;
                } else {
                    dates.push(cur_date.take().unwrap());
                }
            }
            lines[i].date_idx = dates.len();
            cur_date = Some(lines[i].date);
        }
        if let Some(date) = cur_date {
            dates.push(date);
        }
        Ok(Self {
            dates,
            lines,
        })
    }
    pub fn unique_year_month(&self) -> (Option<u16>, Option<u8>) {
        let y1 = self.start_time().year;
        let y2 = self.end_time().year;
        if y1 == y2 {
            let m1 = self.start_time().month;
            let m2 = self.end_time().month;
            if m1 == m2 {
                (Some(y1), Some(m1))
            } else {
                (Some(y1), None)
            }
        } else {
            (None, None)
        }
    }
    pub fn retain_remote_addr_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = IpFilter::new(pattern)?;
        self.lines.retain(|ll| filter.accepts(ll.remote_addr));
        Ok(())
    }
    pub fn retain_methods_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = MethodFilter::from_str(pattern);
        self.lines.retain(|ll| filter.contains(ll.method));
        Ok(())
    }
    pub fn retain_status_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = StatusFilter::from_str(pattern)?;
        self.lines.retain(|ll| filter.accepts(ll.status));
        Ok(())
    }
    pub fn retain_paths_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = StrFilter::new(pattern)?;
        self.lines.retain(|ll| filter.accepts(&ll.path));
        Ok(())
    }
    pub fn retain_referers_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = StrFilter::new(pattern)?;
        self.lines.retain(|ll| filter.accepts(&ll.referer));
        Ok(())
    }
    pub fn retain_dates_matching(&mut self, pattern: &str) -> Result<()> {
        let filter = self.make_date_filter(pattern)?;
        self.lines.retain(|ll| filter.contains(ll.date));
        Ok(())
    }
    pub fn make_date_filter(&self, pattern: &str) -> Result<DateFilter> {
        let (default_year, default_month) = self.unique_year_month();
        Ok(DateFilter::from_arg(pattern, default_year, default_month)?)
    }
    pub fn start_time(&self) -> Date {
        self.lines[0].date
    }
    pub fn end_time(&self) -> Date {
        self.lines[self.lines.len()-1].date
    }
    pub fn day_count(&self) -> usize {
        self.dates.len()
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

fn find_files(path: PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_dir() {
        for entry in path.read_dir()? {
            find_files(entry?.path(), files)?;
        }
    } else if LogFile::is_access_log_path(&path) {
        files.push(path);
    }
    Ok(())
}

fn print_progress(done: usize, total: usize) -> Result<()> {
    let width = 40;
    let p = ProgressBar::new(done as f32 / (total as f32), width);
    let s = format!("{:width$}", p, width=width);
    let mut stderr = io::stderr();
    queue!(stderr, cursor::SavePosition)?;
    queue!(stderr, Clear(ClearType::CurrentLine))?;
    queue!(stderr, Print(format!("{:>4} / {} ", done, total)))?;
    queue!(stderr, PrintStyledContent(style(s).with(Color::Yellow).on(Color::DarkMagenta)))?;
    queue!(stderr, cursor::RestorePosition)?;
    stderr.flush()?;
    Ok(())
}

fn read_files(mut files: Vec<PathBuf>) -> Result<Vec<LogFile>> {
    let total = files.len();
    let mut done = 0;
    print_progress(0, total)?;
    files.drain(..)
        .map(|path| {
            let lf = LogFile::new(path);
            done += 1;
            print_progress(done, total)?;
            lf
        })
        .collect()
}
