use {
    crate::{
        Key,
        Fields,
    },
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

    #[argh(option, default = "Default::default()")]
    /// color and style: 'yes', 'no' or 'auto' (auto should be good in most cases)
    pub color: BoolArg,

    #[argh(option, short = 'k', default = "Default::default()")]
    /// key used in sorting and histogram, either 'hits' (default) or 'bytes'
    pub key: Key,

    #[argh(option, short = 'l', default = "1")]
    /// detail level, from 0 to 6 (default 1), impacts the lengths of tables
    pub length: usize,

    #[argh(option, short = 'f', default = "Default::default()")]
    /// comma separated list of hit fields to display.
    /// use `-f a` to get all fields.
    /// Available fields: date,method,status,ip,ref,path.
    /// Default fields: date,status,ref,path.
    pub fields: Fields,

    #[argh(switch, short = 'c')]
    /// add tables with more popular and less popular entries (ip, referers or paths)
    pub changes: bool,

    #[argh(option, short = 's')]
    /// comma separated list of statuses or status ranges.
    /// (eg: `-s 514` or `-s 4xx,5xx`, or `-s 310-340,400-450` or `-s 5xx,!502`)
    pub status: Option<String>,

    #[argh(option, short = 'm')]
    /// http method to filter by. Make it negative with a `!`.
    /// (eg: `-m PUT` or `-m !DELETE` or `-m none` or `-m other`)
    pub method: Option<String>,

    #[argh(option, short = 'i')]
    /// ip address to filter by. May be negated with a `!`
    pub ip: Option<String>,

    #[argh(option, short = 'd')]
    /// filter the dates, on a precise day or in an inclusive range
    /// (eg: `-r 12/24` or `-r '2021/12/24-2022/01/21'`)
    pub date: Option<String>,

    #[argh(option, short = 'p')]
    /// filter the paths with a pattern
    /// (eg: `-p broot` or `-p '^/\d+'` or `-p 'miaou | blog'`)
    pub path: Option<String>,

    #[argh(switch, short = 'a')]
    /// show all paths, including resources
    pub all: bool,

    #[argh(option, short = 'r')]
    /// filter the referers with a pattern
    pub referer: Option<String>,

    #[argh(switch)]
    /// tries to open all files, whatever their names
    pub no_name_check: bool,

    #[argh(positional)]
    /// the log file or folder to analyze
    pub file: Option<PathBuf>,
}


/// An optional boolean for use in Argh
#[derive(Debug, Clone, Copy, Default)]
pub struct BoolArg(Option<bool>);

impl BoolArg {
    pub fn value(self) -> Option<bool> {
        self.0
    }
}

impl argh::FromArgValue for BoolArg {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "auto" => Ok(BoolArg(None)),
            "yes" => Ok(BoolArg(Some(true))),
            "no" => Ok(BoolArg(Some(false))),
            _ => Err(format!("Illegal value: {:?}", value)),
        }
    }
}
