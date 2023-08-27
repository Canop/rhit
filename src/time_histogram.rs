use {
    crate::*,
    minimad::OwningTemplateExpander,
    termimad::*,
};

static MD: &str = r#"
|:-:|:-:|:-:|:-
|**hour**|**hits**|**bytes**|**${scale}**
|:-|:-:|-:|:-
${bars
|${hour}|${hits}|${bytes}|*${bar}*
}
|-:
"#;

#[derive(Clone)]
struct Bar {
    pub hour: u8,
    pub hits: u64,
    pub bytes_sent: u64,
}

impl Bar {
    pub fn new(hour: u8) -> Self {
        Self {
            hour,
            hits: 0,
            bytes_sent: 0,
        }
    }
}

/// An histogram of hit times.
///
/// There's one bar per hour
#[derive(Clone, Default)]
pub struct TimeHistogram {
    bars: Vec<Bar>,
}

impl TimeHistogram {

    pub fn from(base: &LogBase) -> Self {
        let mut bars: Vec<Bar> = (0..24)
            .map(Bar::new)
            .collect();
        for line in &base.lines {
            let idx = line.time().hour as usize;
            bars[idx].hits += 1;
            bars[idx].bytes_sent += line.bytes_sent;
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
            let value = if printer.key == Key::Hits { bar.hits } else { bar.bytes_sent };
            let part = (value as f32) / max_bar;
            expander.sub("bars")
                .set("hour", bar.hour)
                .set_md("hits", printer.md_hits(bar.hits as usize))
                .set_md("bytes", printer.md_bytes(bar.bytes_sent))
                .set("bar", ProgressBar::new(part, 20));
        }
        printer.print(expander, MD);
    }
    pub fn total_hits(&self) -> u64 {
        self.bars.iter().map(|b| b.hits).sum()
    }
    pub fn total_bytes_sent(&self) -> u64 {
        self.bars.iter().map(|b| b.bytes_sent).sum()
    }
}
