
use argh::FromArgValue;

/// one of the tables that can be displayed
#[derive(Debug, Clone, Copy)]
pub enum Table {
    Dates,
    Status,
    RemoteAddresses,
    Referers,
    Paths,
    Methods,
}

pub static DEFAULT_TABLES: &[Table] = &[
    Table::Dates,
    Table::Status,
    Table::RemoteAddresses,
    Table::Referers,
    Table::Paths,
];

pub static ALL_TABLES: &[Table] = &[
    Table::Dates,
    Table::Methods,
    Table::Status,
    Table::RemoteAddresses,
    Table::Referers,
    Table::Paths,
];

#[derive(Debug, Clone)]
pub struct Tables(Vec<Table>);

impl Default for Tables {
    fn default() -> Self {
        Self (DEFAULT_TABLES.to_vec())
    }
}

impl Tables {
    fn all() -> Self {
        Self (ALL_TABLES.to_vec())
    }
}

impl IntoIterator for Tables {
    type Item = Table;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromArgValue for Tables {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        let value = value.to_lowercase();
        if value == "all" || value == "a" {
            return Ok(Self::all());
        }
        let mut v = Vec::new();
        for s in value.split(',') {
            if s.contains("date") {
                v.push(Table::Dates);
            } else if s.contains("stat") {
                v.push(Table::Status);
            } else if s.contains("addr") {
                v.push(Table::RemoteAddresses);
            } else if s.contains("ref") {
                v.push(Table::Referers);
            } else if s.contains("path") {
                v.push(Table::Paths);
            } else if s.contains("method") {
                v.push(Table::Methods);
            } else {
                return Err(format!("Unrecognized table : {:?}", s));
            }
        }
        Ok(Self(v))
    }
}
