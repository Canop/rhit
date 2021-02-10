use {
    crate::*,
};

/// A non empty group of lines, with a common characteristic,
/// for stats
pub struct LineGroup<'b> {
    pub lines: Vec<&'b LogLine>, // guaranteed not empty
    pub trend: Trend,
}

impl<'b> LineGroup<'b> {
    pub fn new(
        lines: Vec<&'b LogLine>,
        trend_computer: &TrendComputer,
    ) -> Self {
        let trend = trend_computer.compute_trend(&lines);
        Self {
            lines,
            trend,
        }
    }
    pub fn any(&self) -> &LogLine {
        &self.lines[0]
    }
    pub fn hits(&self) -> usize {
        self.lines.len()
    }
}
