use {
    anyhow::Result,
};

/// one of the tables that can be displayed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Dates,
    Methods,
    Status,
    Ip,
    Referers,
    Paths,
}

pub static DEFAULT_FIELDS: &[Field] = &[
    Field::Dates,
    Field::Status,
    Field::Referers,
    Field::Paths,
];

pub static ALL_FIELDS: &[Field] = &[
    Field::Dates,
    Field::Methods,
    Field::Status,
    Field::Ip,
    Field::Referers,
    Field::Paths,
];

#[derive(Debug, Clone, PartialEq)]
pub struct Fields(pub Vec<Field>);

impl Default for Fields {
    fn default() -> Self {
        Self (DEFAULT_FIELDS.to_vec())
    }
}

impl Fields {
    fn empty() -> Self {
        Self(Vec::new())
    }
    fn all() -> Self {
        Self(ALL_FIELDS.to_vec())
    }
    pub fn contains(&self, tbl: Field) -> bool {
        self.0.contains(&tbl)
    }
    pub fn remove(&mut self, removed: Field) {
        self.0.retain(|&f| f!=removed);
    }
    // add a field, preventing duplicates
    // (may be used when the field is present to reorder)
    pub fn add(&mut self, added: Field) {
        self.remove(added);
        self.0.push(added);
    }
}

impl IntoIterator for Fields {
    type Item = Field;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Error)]
pub enum ParseFieldError {

    #[error("unrecognized field start {0:?}")]
    UnrecognizedFieldStart(char),
}

impl FromStr for Fields {
    type Err = ParseFieldError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        //let additive = value.contains('+') || value.contains('-');
        let mut fields = if value.starts_with('+') || value.starts_with('-') {
            // if it starts with an addition or removal, the default set is implied
            Fields::default()
        } else {
            Fields::empty()
        };
        let mut skip_alpha = false;
        let mut negative = false;
        for c in value.chars() {
            match c.to_ascii_lowercase() {
                '+' | ' ' | ',' => {
                    skip_alpha = false;
                    negative = false;
                }
                '-' => {
                    skip_alpha = false;
                    negative = true;
                }
                'a' if !skip_alpha => {
                    if negative {
                        fields = Fields::empty();
                    } else {
                        fields = Fields::all();
                    }
                    skip_alpha = true;
                }
                c if !skip_alpha => {
                    let field = match c {
                        'd' => Field::Dates,
                        's' => Field::Status,
                        'a'|'i' => Field::Ip,
                        'r' => Field::Referers,
                        'p' => Field::Paths,
                        'm' => Field::Methods,
                        _ => {
                            return Err(ParseFieldError::UnrecognizedFieldStart(c));
                        }
                    };
                    if negative {
                        fields.remove(field);
                    } else {
                        fields.add(field);
                    }
                    skip_alpha = true;
                }
                _ => {}
            }
        }
        Ok(fields)
    }
}

#[cfg(test)]
mod fields_parsing_tests {
    use {
        super::*,
        super::Field::*,
    };

    #[test]
    fn parse_fields_explicit() {
        assert_eq!(
            Fields::from_arg_value("paths").unwrap(),
            Fields(vec![Paths]),
        );
        assert_eq!(
            Fields::from_arg_value("p").unwrap(),
            Fields(vec![Paths]),
        );
        assert_eq!(
            Fields::from_arg_value("ip,date,ref").unwrap(),
            Fields(vec![Ip, Dates, Referers]),
        );
        assert_eq!(
            Fields::from_arg_value("ip+date+ref").unwrap(),
            Fields(vec![Ip, Dates, Referers]),
        );
        assert_eq!(
            Fields::from_arg_value("i,d,ref").unwrap(),
            Fields(vec![Ip, Dates, Referers]),
        );
        assert_eq!(
            Fields::from_arg_value("i+d+r").unwrap(),
            Fields(vec![Ip, Dates, Referers]),
        );
        assert_eq!(
            Fields::from_arg_value("method,status,ip,date,ref").unwrap(),
            Fields(vec![Methods, Status, Ip, Dates, Referers]),
        );
    }

    #[test]
    fn parse_fields_no_duplicate() {
        assert_eq!(
            Fields::from_arg_value("paths,p").unwrap(),
            Fields(vec![Paths]),
        );
        assert_eq!(
            Fields::from_arg_value("referer,method,status,ip,date,ref").unwrap(),
            Fields(vec![Methods, Status, Ip, Dates, Referers]),
        );
    }

    #[test]
    fn parse_fields_all() {
        assert_eq!(
            Fields::from_arg_value("a").unwrap(),
            Fields(ALL_FIELDS.to_vec()),
        );
        assert_eq!(
            Fields::from_arg_value("all").unwrap(),
            Fields(ALL_FIELDS.to_vec()),
        );
    }

    #[test]
    fn parse_fields_add_remove_to_default() {
        assert_eq!(
            Fields::from_arg_value("+r+i").unwrap(),
            Fields(vec![Dates, Status, Paths, Referers, Ip]),
        );
        assert_eq!(
            Fields::from_arg_value("+s,m").unwrap(),
            Fields(vec![Dates, Referers, Paths, Status, Methods]),
        );
        assert_eq!(
            Fields::from_arg_value("+ip-path").unwrap(),
            Fields(vec![Dates, Status, Referers, Ip]),
        );
        assert_eq!(
            Fields::from_arg_value("-p+i,").unwrap(),
            Fields(vec![Dates, Status, Referers, Ip]),
        );
        assert_eq!(
            Fields::from_arg_value("+i,").unwrap(),
            Fields(vec![Dates, Status, Referers, Paths, Ip]),
        );
        assert_eq!(
            Fields::from_arg_value("-date-p").unwrap(),
            Fields(vec![Status, Referers]),
        );
        assert_eq!(
            Fields::from_arg_value("-d-p+i+p+m").unwrap(),
            Fields(vec![Status, Referers, Ip, Paths, Methods]),
        );
    }

    #[test]
    fn parse_fields_algebric_no_default() {
        assert_eq!(
            Fields::from_arg_value("all+ref+i").unwrap(),
            Fields(vec![Dates, Methods, Status, Paths, Referers, Ip]),
        );
        assert_eq!(
            Fields::from_arg_value("all-ref-i").unwrap(),
            Fields(vec![Dates, Methods, Status, Paths]),
        );
        assert_eq!(
            Fields::from_arg_value("s-m").unwrap(),
            Fields(vec![Status]),
        );
        assert_eq!(
            Fields::from_arg_value("all-i,").unwrap(),
            Fields(vec![Dates, Methods, Status, Referers, Paths]),
        );
        assert_eq!(
            Fields::from_arg_value("all-date-p").unwrap(),
            Fields(vec![Methods, Status, Ip, Referers]),
        );
    }

}
