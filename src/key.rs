
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

impl argh::FromArgValue for Key {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "h" | "hit" | "hits" => Ok(Self::Hits),
            "b" | "byte" | "bytes"  => Ok(Self::Bytes),
            _ => Err(format!("Illegal value: {:?}", value)),
        }
    }
}
