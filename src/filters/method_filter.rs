use {
    crate::*,
};

/// A filter for HTTP methods
#[derive(Debug, Clone, Copy)]
pub struct MethodFilter {
    negative: bool,
    method: Method,
}

impl MethodFilter {
    pub fn from_string(mut pattern: &str) -> Self {
        let negative = pattern.starts_with('!');
        if negative {
            pattern = &pattern[1..];
        }
        let pattern = pattern.trim();
        let method = Method::from(pattern);
        Self { negative, method }
    }
    pub fn contains(self, candidate: Method) -> bool {
        if self.negative {
            self.method != candidate
        } else {
            self.method == candidate
        }
    }
}

