use argh::FromArgValue;

/// one of the tables that can be displayed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Dates,
    Status,
    RemoteAddresses,
    Referers,
    Paths, // popular paths
    Methods,
}

pub static DEFAULT_TABLES: &[Field] = &[
    Field::Dates,
    Field::Status,
    Field::Referers,
    Field::Paths,
];

pub static ALL_TABLES: &[Field] = &[
    Field::Dates,
    Field::Methods,
    Field::Status,
    Field::RemoteAddresses,
    Field::Referers,
    Field::Paths,
];

#[derive(Debug, Clone)]
pub struct Fields(Vec<Field>);

impl Default for Fields {
    fn default() -> Self {
        Self (DEFAULT_TABLES.to_vec())
    }
}

impl Fields {
    fn all() -> Self {
        Self (ALL_TABLES.to_vec())
    }
    pub fn contains(&self, tbl: Field) -> bool {
        self.0.contains(&tbl)
    }
}

impl IntoIterator for Fields {
    type Item = Field;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromArgValue for Fields {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        let value = value.to_lowercase();
        if value == "all" || value == "a" {
            return Ok(Self::all());
        }
        let mut v = Vec::new();
        for s in value.split(',') {
            if s.contains("date") {
                v.push(Field::Dates);
            } else if s.contains("stat") {
                v.push(Field::Status);
            } else if s.contains("addr") || s.contains("ip") {
                v.push(Field::RemoteAddresses);
            } else if s.contains("ref") {
                v.push(Field::Referers);
            } else if s.contains("path") {
                v.push(Field::Paths);
            } else if s.contains("method") {
                v.push(Field::Methods);
            } else {
                return Err(format!("Unrecognized table : {:?}", s));
            }
        }
        Ok(Self(v))
    }
}
