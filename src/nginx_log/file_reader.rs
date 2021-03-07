use {
    crate::*,
    anyhow::*,
    crossterm::{
        self,
        cursor,
        execute,
        style::{style, Color, Print, PrintStyledContent},
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

pub fn get_file_first_date(path: &Path) -> Result<Date> {
    debug!("reading date in file {:?}", &path);
    let file = File::open(path)?;
    if path.extension().and_then(|e| e.to_str()) == Some("gz") {
        let file = BufReader::new(file);
        read_first_date(GzDecoder::new(file))
    } else {
        read_first_date(file)
    }
}
fn read_first_date<R: Read>(file: R) -> Result<Date> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    if reader.read_line(&mut line)? < 20 {
        bail!("File too short");
    }
    let log_line = LogLine::from_str(&line)?;
    Ok(log_line.date)
}

pub struct FileReader<'c, C>
where
    C: LineConsumer
{
    root: PathBuf,
    filterer: Filterer,
    consumer: &'c mut C,
    paths: Vec<PathBuf>,
    stop_on_error: bool,
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
        path: &Path,
        args: &args::Args,
        consumer: &'c mut C,
    ) -> Result<Self> {
        let check_names = !args.no_name_check;
        let ff = FileFinder::new(path.to_path_buf(), check_names);
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
            root: path.into(),
            filterer,
            consumer,
            paths,
            stop_on_error,
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
        print_progress(0, total)?;
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
            print_progress(done, total)?;
        }
        execute!(io::stderr(), Clear(ClearType::CurrentLine))?;
        eprintln!("I've read {} files in {:?}", total, self.root);
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
