use {
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

