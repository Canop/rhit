use {
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(Debug, FromArgs)]
/// I need to explain this, I guess
///
/// Source at https://github.com/Canop/csv2svg
pub struct Args {

    #[argh(switch, short = 'v')]
    /// print the version
    pub version: bool,

    #[argh(option, short = 'p')]
    /// filter the paths with a pattern
    pub pattern: Option<String>,

    #[argh(positional)]
    /// the log file or folder to analyze
    pub path: Option<PathBuf>,
}

