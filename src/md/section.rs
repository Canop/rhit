
#[derive(Debug, Clone, Copy)]
pub enum View {
    Full, // we show all elements (status, methods)
    Limited(usize),
}

impl View {
    pub fn limit(self) -> usize {
        match self {
            Self::Full => 100,
            Self::Limited(limit) => limit,
        }
    }
}

/// describe how the table(s) related to a hit field
/// must be printed
pub struct Section {
    pub groups_name: &'static str,
    pub group_key: &'static str,
    pub view: View,
    pub changes: bool, // means it may sense to show changes tables
}
