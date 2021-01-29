pub use {
    crate::*,
    anyhow::*,
    chrono::{self, DateTime, Duration, FixedOffset, TimeZone},
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
    pub lines: Vec<LogLine>,
}

impl LogBase {
    pub fn new(path: &Path) -> Result<Self> {
        let mut files = Vec::new();
        find_files(path.to_path_buf(), &mut files)?;
        let mut log_files = read_files(files)?;
        execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
        if log_files.is_empty() {
            bail!("no log file found in {:?}", path);
        } else {
            println!("I've read {} files in {:?}", log_files.len(), path);
        }
        log_files.sort_by_key(LogFile::start_time);
        let mut lines = Vec::new();
        for (a, b) in log_files.iter().tuple_windows() {
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
        for mut file in log_files {
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
    pub fn retain_referers_matching(&mut self, pattern: &Regex) {
        self.lines.retain(|ll| pattern.is_match(&ll.referer));
    }
    pub fn start_time(&self) -> DateTime<FixedOffset> {
        self.lines[0].time_local
    }
    pub fn end_time(&self) -> DateTime<FixedOffset> {
        self.lines[self.lines.len()-1].time_local
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
    let mut stdout = io::stdout();
    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, Clear(ClearType::CurrentLine))?;
    queue!(stdout, Print(format!("{:>4} / {} ", done, total)))?;
    queue!(stdout, PrintStyledContent(style(s).with(Color::Yellow).on(Color::DarkMagenta)))?;
    queue!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;
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
