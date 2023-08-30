use {
    crate::{
        Key,
        Fields,
        Output,
    },
    clap::{Parser, ValueEnum},
    std::path::PathBuf,
    termimad::crossterm::tty::IsTty,
};

/// Program launch argument
#[derive(Debug, Default, Parser)]
#[command(author, about, name = "rhit", disable_version_flag = true, version, disable_help_flag = true)]
pub struct Args {

    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    /// Whether to have styles and colors
    #[arg(long, default_value="auto", value_name = "color")]
    pub color: TriBool,

    /// Key used in sorting and histogram, either `hits` or `bytes`
    #[arg(short, long, default_value="hits")]
    pub key: Key,

    /// Detail level, from `0` to `6`, impacts the lengths of tables
    #[arg(short, long, default_value = "1")]
    pub length: usize,

    /// Comma separated list of hit fields to display.
    /// Use `-f a` to get all fields.
    /// Use `-f +i` to add ip.
    /// Available fields: `date,time,method,status,ip,ref,path`.
    #[arg(short, long, default_value = "date,status,ref,path")]
    pub fields: Fields,

    /// Add tables with more popular and less popular entries (ip, referers and paths)
    #[arg(short, long)]
    pub changes: bool,

    /// Filter the dates on a precise day or in an inclusive range
    /// (eg: `-d 12/24` or `-d '2021/12/24-2022/01/21'`)
    #[arg(short, long)]
    pub date: Option<String>,

    /// Ip address to filter by. May be negated with a `!`
    #[arg(short, long)]
    pub ip: Option<String>,

    /// HTTP method to filter by. Make it negative with a `!`.
    /// (eg: `-m PUT` or `-m !DELETE` or `-m none` or `-m other`)
    #[arg(short, long)]
    pub method: Option<String>,

    /// Pattern for path filtering
    /// (eg: `-p broot` or `-p '^/\d+'` or `-p 'miaou | blog'`)
    #[arg(short, long)]
    pub path: Option<String>,

    /// Referrer filter
    #[arg(short, long)]
    pub referer: Option<String>,

    /// Comma separated list of statuses or status ranges to filter by
    /// (eg: `-s 514` or `-s 4xx,5xx`, or `-s 310-340,400-450` or `-s 5xx,!502`)
    #[arg(short, long)]
    pub status: Option<String>,

    /// Filter the time of the day, in the logs' timezone
    /// (eg: `-t '>19:30'` to get evening hits)
    #[arg(short, long)]
    pub time: Option<String>,

    /// Show all paths, including resources
    #[arg(short, long)]
    pub all: bool,

    /// Try to open all files, whatever their names
    #[arg(long)]
    pub no_name_check: bool,

    /// Output: by default pretty summary tables but you can also
    /// output log lines as `csv`, `json`, or `raw` (as they appear in the log files)
    #[arg(short, long, default_value="tables")]
    pub output: Output,

    /// Don't print anything during load: no progress bar or file list
    #[arg(long)]
    pub silent_load: bool,

    /// The log file or folder to analyze. It not provided, logs will be opened
    /// at their standard location
    pub files: Vec<PathBuf>,
}

#[derive(ValueEnum)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TriBool {
    #[default]
    Auto,
    Yes,
    No,
}
impl TriBool {
    pub fn unwrap_or_else<F>(self, f: F) -> bool
    where
        F: FnOnce() -> bool
    {
        match self {
            Self::Auto => f(),
            Self::Yes => true,
            Self::No => false,
        }
    }
}

impl Args {
    pub fn color(&self) -> bool {
        self.color.unwrap_or_else(|| std::io::stdout().is_tty())
    }
}
