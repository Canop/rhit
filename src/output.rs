use {
    anyhow::Result,
    argh::FromArgValue,
};

/// Kind of output
///
/// TODO:
/// - JSON
#[derive(Debug, Clone, Copy, Default)]
pub enum Output {
    /// log lines exactly as they appear in the ngnix log files
    Raw,
    /// A set of colored tables
    #[default]
    Tables,
    /// Comma separated values, one row for the header then
    /// one row per log line
    Csv,
    /// An array of log objects
    Json,
}

impl FromArgValue for Output {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "r" | "raw" => Ok(Self::Raw),
            "t" | "tbl" | "tables" => Ok(Self::Tables),
            "c" | "csv" => Ok(Self::Csv),
            "j" | "json" => Ok(Self::Json),
            _ => Err(format!("unrecognized output : {value:?}")),
        }
    }
}
