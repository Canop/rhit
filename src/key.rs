use {
    std::str::FromStr,
    thiserror::Error,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Hits,
    Bytes,
}

impl Default for Key {
    fn default() -> Self {
        Self::Hits
    }
}

#[derive(Debug, Error)]
pub enum ParseKeyError {
    #[error("unrecognized key {0:?}")]
    UnrecognizedKey(String),
}

impl FromStr for Key {
    type Err = ParseKeyError;
    fn from_str(value: &str) -> Result<Self, ParseKeyError> {
        match value.to_lowercase().as_ref() {
            "h" | "hit" | "hits" => Ok(Self::Hits),
            "b" | "byte" | "bytes"  => Ok(Self::Bytes),
            _ => Err(ParseKeyError::UnrecognizedKey(value.to_owned()))
        }
    }
}
