use {
    crate::*,
    minimad::OwningTemplateExpander,
    termimad::*,
};

static MD: &str = r#"
|:-|-:|:-:|:-
|**date**|**bytes**|**hits**|**${scale}**
|:-|-:|-:|:-
${bars
|${date}|${bytes-sent}|${hits}|*${bar}*
}
|-:
"#;

pub struct Bar {
    date: Date,
    sum_bytes_sent: u64,
    count: usize,
}

pub struct Histogram {
    pub bars: Vec<Bar>,
}
impl Histogram {
    pub fn from(base: &LogBase) -> Self {
        let mut bars: Vec<Bar> = base.dates.iter()
            .map(|&date| Bar { date, sum_bytes_sent: 0, count: 0 })
            .collect();
        for line in &base.lines {
            bars[line.date_idx].count += 1;
            bars[line.date_idx].sum_bytes_sent += line.bytes_sent;
        }
        Self { bars }
    }
    /// compute the counts per day of lines
    pub fn line_counts(base: &LogBase, lines: &[&LogLine]) -> Vec<usize> {
        let mut counts = vec![0; base.dates.len()];
        for line in lines {
            counts[line.date_idx] += 1;
        }
        counts
    }
    pub fn print(&self, printer: &md::Printer) {
        let mut expander = OwningTemplateExpander::new();
        let max_hits = self.bars.iter().map(|b| b.count).max().unwrap();
        expander.set(
            "scale",
            format!("0               {:>4}", file_size::fit_4(max_hits as u64)),
        );
        let max_hits = max_hits as f32;
        for bar in &self.bars {
            let part = (bar.count as f32) / max_hits;
            expander.sub("bars")
                .set("date", bar.date)
                .set("bytes-sent", file_size::fit_4(bar.sum_bytes_sent))
                .set("hits", bar.count)
                .set("bar", ProgressBar::new(part, 20));
        }
        printer.print(expander, MD);
    }
}
