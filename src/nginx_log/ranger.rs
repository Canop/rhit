use {
    crate::*,
    std::{
        str::CharIndices,
    },
};

pub struct Ranger<'s> {
    s: &'s str,
    char_indices: CharIndices<'s>,
    pos: usize,
    last: Option<char>,
}
impl<'s> Ranger<'s> {
    pub fn new(s: &'s str) -> Self {
        Self {
            s,
            char_indices: s.char_indices(),
            pos: 0,
            last: None,
        }
    }
    pub fn until(&mut self, end: char) -> Result<&'s str, LogParseError> {
        for (idx, c) in &mut self.char_indices {
            if c == end {
                let start = self.pos;
                self.pos = idx;
                self.last = Some(c);
                return Ok(&self.s[start..self.pos]);
            }
        }
        Err(LogParseError::CharNotFound(end))
    }
    pub fn between(&mut self, start: char, end: char) -> Result<&'s str, LogParseError> {
        if Some(start) == self.last {
            self.pos += start.len_utf8();
            return self.until(end);
        }
        for (idx, c) in &mut self.char_indices {
            if c == start {
                let pos = idx + start.len_utf8();
                for (idx, c) in &mut self.char_indices {
                    if c == end {
                        self.pos = idx;
                        self.last = Some(c);
                        return Ok(&self.s[pos..self.pos]);
                    }
                }
                return Err(LogParseError::CharNotFound(end));
            }
        }
        Err(LogParseError::CharNotFound(start))
    }
}
