use {
    crate::Tables,
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(Debug, FromArgs)]
/// Rhit gives you a report of the hits found in your nginx logs.
///
/// Source at https://github.com/Canop/rhit
pub struct Args {

    #[argh(switch)]
    /// print the version
    pub version: bool,

    #[argh(option, short = 'l', default = "1")]
    /// detail level, from 0 to 6 (default 1), impacts the lengths of tables
    pub length: usize,

    #[argh(option, short = 't', default = "Default::default()")]
    /// tables to display: comma separated list of tables (default all but methods).
    /// use `-t a` to get all tables.
    /// Available tables: date,status,method,addr,ref,path,trend
    pub tables: Tables,

    #[argh(option, short = 's')]
    /// comma separated list of statuses or status range.
    /// (eg: `-s 514` or `-s 4xx,5xx`, or `-s 310-340,400-450` or `-s 5xx`)
    pub status: Option<String>,

    #[argh(option, short = 'm')]
    /// http method to filter by. Make it negative with a `!`.
    /// (eg: `-m PUT` or `-m !DELETE` or `-m none` or `-m other`)
    pub method: Option<String>,

    #[argh(option, short = 'a')]
    /// ip address to filter by. May be negated with a `!`
    pub addr: Option<String>,

    #[argh(option, short = 'd')]
    /// filter the dates, on a precise day or in an inclusive range
    /// (eg: `-r 12/24` or `-r '2021/12/24-2022/01/21'`)
    pub date: Option<String>,

    #[argh(option, short = 'p')]
    /// filter the paths with a pattern
    /// (eg: `-p broot` or `-p '^/\d+'` or `-p 'miaou | blog'`)
    pub path: Option<String>,

    #[argh(option, short = 'r')]
    /// filter the referers with a pattern
    pub referer: Option<String>,

    #[argh(positional)]
    /// the log file or folder to analyze
    pub file: Option<PathBuf>,
}

