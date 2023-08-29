use {
    std::str::FromStr,
    thiserror::Error,
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

#[derive(Debug, Error)]
pub enum ParseOutputError {
    #[error("unrecognized output {0:?}")]
    UnrecognizedValue(String),
}

impl FromStr for Output {
    type Err = ParseOutputError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "r" | "raw" => Ok(Self::Raw),
            "t" | "tbl" | "tables" => Ok(Self::Tables),
            "c" | "csv" => Ok(Self::Csv),
            "j" | "json" => Ok(Self::Json),
            _ => Err(ParseOutputError::UnrecognizedValue(value.to_owned()))
        }
    }
}
