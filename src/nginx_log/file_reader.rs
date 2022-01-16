use {
    crate::*,
    anyhow::{bail, Result},
    crossterm::{
        self,
        cursor,
        execute,
        style::{style, Color, Print, PrintStyledContent, Stylize},
        terminal::{Clear, ClearType},
        queue,
    },
    flate2::bufread::GzDecoder,
    std::{
        fs::File,
        io::{self, BufRead, BufReader, Read, Write},
        path::{Path, PathBuf},
        str::FromStr,
    },
    termimad::ProgressBar,
};

pub fn get_file_first_date(path: &Path) -> Result<Option<Date>> {
    debug!("reading date in file {:?}", &path);
    let file = File::open(path)?;
    if path.extension().and_then(|e| e.to_str()) == Some("gz") {
        let file = BufReader::new(file);
        read_first_date(GzDecoder::new(file))
    } else {
        read_first_date(file)
    }
}
fn read_first_date<R: Read>(file: R) -> Result<Option<Date>> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    // a log file may contain non log lines, for example when
    // logrotate added its own traces.
    // See https://github.com/Canop/rhit/issues/8
    // We'll try up to 3 lines
    for _ in 0..3 {
        let len = reader.read_line(&mut line)?;
        if len < 20 {
            if len == 0 { // EOF
                return Ok(None);
            }
            debug!("line too short"); // doesn't contain a log
            continue;
        }
        match LogLine::from_str(&line) {
            Ok(l) => {
                return Ok(Some(l.date));
            }
            _ => {
                debug!("skipping line {:?}", &line);
            }
        }
        line.clear();
    }
    Ok(None)
}

pub struct FileReader<'c, C>
where
    C: LineConsumer
{
    roots: Box<[PathBuf]>,
    filterer: Filterer,
    consumer: &'c mut C,
    paths: Vec<PathBuf>,
    stop_on_error: bool,
    silent: bool,
}

pub trait LineConsumer {
    fn start_eating(
        &mut self,
        first_date: Date,
    );
    fn eat_line(
        &mut self,
        log_line: LogLine,
        raw_line: &str,
        filtered_out: bool,
    );
}

impl<'c, C: LineConsumer> FileReader<'c, C> {
    pub fn new(
        paths: &[PathBuf],
        args: &args::Args,
        consumer: &'c mut C,
    ) -> Result<Self> {
        let check_names = !args.no_name_check;
        let roots = paths.to_vec().into_boxed_slice();
        let ff = FileFinder::new(&roots, check_names);
        let mut dated_files = time!(ff.dated_files())?;
        if dated_files.is_empty() {
            bail!("no log file found");
        }
        let first_date = dated_files[0].0;
        let last_date = dated_files[dated_files.len()-1].0; // last first date
        let filterer = Filterer::new(args, first_date, last_date)?;
        let paths: Vec<PathBuf> = dated_files.drain(..).map(|df| df.1).collect();
        let stop_on_error = check_names;
        consumer.start_eating(first_date);
        Ok(Self {
            roots,
            filterer,
            consumer,
            paths,
            stop_on_error,
            silent: args.silent_load,
        })
    }
    pub fn filterer(self) -> Filterer {
        self.filterer
    }
    pub fn read_all_files(
        &mut self,
    ) -> Result<()> {
        let total =  self.paths.len();
        let mut done = 0;
        if !self.silent {
            print_progress(0, total)?;
        }
        let paths = std::mem::take(&mut self.paths);
        for path in paths {
            if let Err(e) = self.read_file_lines(&path) {
                if self.stop_on_error {
                    return Err(e);
                } else {
                    warn!("Error while reading file: {}", e);
                }
            }
            done += 1;
            if !self.silent {
                print_progress(done, total)?;
            }
        }
        execute!(io::stderr(), Clear(ClearType::CurrentLine))?;
        if !self.silent {
            // if we're here, total, which is the count of log files, is at least 1
            let roots_string = if self.roots.len() == 1 {
                format!("{:?}", self.roots[0])
            } else {
                format!("{:?}", self.roots)
            };
            eprintln!(
                "I've read {} file{} in {}",
                total,
                if total > 1 { "s" } else { "" },
                roots_string,
            );
        }
        Ok(())
    }
    fn read_file_lines(&mut self, path: &Path) -> Result<()> {
        let file = File::open(&path)?;
        if path.extension().and_then(|e| e.to_str()) == Some("gz") {
            let file = BufReader::new(file);
            self.read_lines(GzDecoder::new(file), path)
        } else {
            self.read_lines(file, path)
        }
    }
    fn read_lines<R: Read>(&mut self, file: R, path: &Path) -> Result<()> {
        debug!("reading file {:?}", path);
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut errors = 0;
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break; // EOF
            }
            match LogLine::from_str(&line) {
                Ok(log_line) => {
                    let filtered_out = !self.filterer.accepts(&log_line);
                    self.consumer.eat_line(log_line, &line, filtered_out);
                }
                Err(e) => {
                    // we only log the first error
                    match errors {
                        0 => warn!("{} in {}", e, line),
                        1 => {
                            warn!("logging other errors in this file as debug only");
                            debug!("{} in {}", e, line);
                        }
                        _ => {
                            debug!("{} in {}", e, line);
                        }
                    }
                    errors += 1;
                }
            }
        }
        if errors > 0 {
            warn!("{} errors in {:?}", errors, &path);
        }
        Ok(())
    }
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
