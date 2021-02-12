use {
    crate::*,
    minimad::OwningTemplateExpander,
    termimad::*,
};

static MD: &str = r#"
|:-:|:-:|:-:|:-
|**date**|**hits**|**bytes**|**${scale}**
|:-|:-:|-:|:-
${bars
|${date}|${hits}|${bytes}|*${bar}*
}
|-:
"#;

pub struct Bar {
    date: Date,
    hits: u64,
    bytes_sent: u64,
}

pub struct Histogram {
    pub bars: Vec<Bar>,
}

impl Histogram {

    pub fn from(base: &LogBase) -> Self {
        let mut bars: Vec<Bar> = base.dates.iter()
            .map(|&date| Bar { date, bytes_sent: 0, hits: 0 })
            .collect();
        for line in &base.lines {
            bars[line.date_idx].hits += 1;
            bars[line.date_idx].bytes_sent += line.bytes_sent;
        }
        Self { bars }
    }

    pub fn print(
        &self,
        printer: &md::Printer,
    ) {
        let mut expander = OwningTemplateExpander::new();
        let max_bar = self.bars
            .iter()
            .map(|b| if printer.key==Key::Hits { b.hits } else { b.bytes_sent })
            .max().unwrap();
        expander.set(
            "scale",
            format!("0               {:>4}", file_size::fit_4(max_bar)),
        );
        let max_bar = max_bar as f32;
        for bar in &self.bars {
            if printer.date_filter.map_or(true, |f| f.contains(bar.date)) {
                let value = if printer.key == Key::Hits { bar.hits } else { bar.bytes_sent };
                let part = (value as f32) / max_bar;
                expander.sub("bars")
                    .set("date", bar.date)
                    .set_md("hits", printer.md_hits(bar.hits as usize))
                    .set_md("bytes", printer.md_bytes(bar.bytes_sent))
                    .set("bar", ProgressBar::new(part, 20));
            }
        }
        printer.print(expander, MD);
    }
}
