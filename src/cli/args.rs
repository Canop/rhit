use {
    crate::Tables,
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(Debug, FromArgs)]
/// rhit gives you a small report of the hits found in your nginx logs.
///
/// Source at https://github.com/Canop/rhit
pub struct Args {

    #[argh(switch, short = 'v')]
    /// print the version
    pub version: bool,

    #[argh(option, short = 'l', default = "1")]
    /// detail level, from 0 to 6 (default 1), impacts the lengths of tables
    pub length: usize,

    #[argh(option, short = 't', default = "Default::default()")]
    /// tables to display (default all): comma separated list of tables.
    /// Available tables: date,status,addr,ref,path
    pub tables: Tables,

    #[argh(option, short = 'd')]
    /// filter the dates, on a precise day or in an inclusive range
    /// (eg: `-r 12/24` or `-r "2021/12/24-2022/01/21"`)
    pub date: Option<String>,

    #[argh(option, short = 'p')]
    /// filter the paths with a pattern
    pub path: Option<String>,

    #[argh(option, short = 'r')]
    /// filter the referers with a pattern
    pub referer: Option<String>,

    #[argh(positional)]
    /// the log file or folder to analyze
    pub file: Option<PathBuf>,
}

