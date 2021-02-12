use {
    super::*,
    crate::*,
    have::Fun,
    minimad::{OwningTemplateExpander, TextTemplate},
    num_format::{Locale, ToFormattedString, WriteFormatted},
    std::{
        cmp::Reverse,
        fmt::Display,
        hash::Hash,
    },
    termimad::*,
};

static MD_GROUPS_NO_TRENDS: &str = r#"
## ${groups-count} ${groups-name}. ${limited}
|:-:|:-|:-:|:-:|:-:
|**#**|**${group-key}**|**hits**|**%**|**bytes**
|-:|:-|-:|-:|-:
${groups
|${idx}|${group-value}|${hits}|${percent}|${bytes}
}
|-:
"#;

static MD_GROUPS_TRENDS: &str = r#"
## ${groups-count} ${groups-name}. ${limited}
|:-:|:-|:-:|:-:|:-:|:-:|:-:
|**#**|**${group-key}**|**hits**|**%**|**bytes**|**days**|**trend**
|-:|:-|-:|-:|-:|-:|:-:|
${groups
|${idx}|${group-value}|${hits}|${percent}|${bytes}|*${histo_line}*|${trend}
}
|-:
"#;

static MD_GROUPS_TRENDS_NO_ROW_IDX: &str = r#"
## ${groups-count} ${groups-name}. ${limited}
|:-|:-:|:-:|:-:|:-:|:-:
|**${group-key}**|**hits**|**%**|**bytes**|**days**|**trend**
|:-:|-:|-:|-:|-:|:-:|
${groups
|${group-value}|${hits}|${percent}|${bytes}|*${histo_line}*|${trend}
}
|-:
"#;

pub struct Printer {
    pub skin: MadSkin,
    pub tables: Tables,
    pub terminal_width: usize,
    pub detail_level: usize,
    pub key: Key,
    pub date_filter: Option<DateFilter>,
}

impl Printer {
    pub fn new(args: &args::Args, log_base: &LogBase) -> Self {
        let detail_level = args.length;
        let tables = args.tables.clone();
        let terminal_width = terminal_size().0 as usize;
        let color = args.color.value().unwrap_or(!is_output_piped());
        let skin = skin::make_skin(color);
        let key = args.key;
        let date_filter = args.date.as_ref()
            .and_then(|p| log_base.make_date_filter(p).ok());
        Self { skin, tables, terminal_width, detail_level, key, date_filter }
    }
    pub fn print(
        &self,
        expander: OwningTemplateExpander,
        template: &str,
    ) {
        let template = TextTemplate::from(template);
        let text = expander.expand(&template);
        let fmt_text = FmtText::from_text(&self.skin, text, Some(self.terminal_width));
        print!("{}", fmt_text);
    }
    pub fn md_hits(&self, hits: usize) -> String {
        match self.key {
            Key::Hits => {
                let mut s = "*".to_string();
                s.write_formatted(&hits, &Locale::en).unwrap();
                s.push('*');
                s
            }
            Key::Bytes => hits.to_formatted_string(&Locale::en),
        }
    }
    pub fn md_bytes(&self, bytes: u64) -> String {
        let s = file_size::fit_4(bytes);
        match self.key {
            Key::Hits => s,
            Key::Bytes => format!("*{}*", s),
        }
    }

    pub fn print_groups<'b, T, F, G>(
        &self,
        groups_name: &str,
        group_key: &str,
        log_lines: &'b [LogLine],
        filter: &F,
        grouper: &G,
        trend_computer: Option<&TrendComputer>,
        limit: usize,
        row_idx: bool,
    ) where
        T: Display + Hash + Eq + 'b,
        F: Fn(&&LogLine) -> bool,
        G: for<'a> Fn(&'a &'b LogLine) -> T,
    {
        if let Some(trend_computer) = trend_computer {
            self.print_groups_trends(
                groups_name,
                group_key,
                log_lines,
                filter,
                grouper,
                trend_computer,
                limit,
                row_idx,
            );
        } else {
            self.print_groups_no_trends(
                groups_name,
                group_key,
                log_lines,
                filter,
                grouper,
                limit,
            );
        }
    }

    pub fn print_groups_no_trends<'b, T, F, G>(
        &self,
        groups_name: &str,
        group_key: &str,
        log_lines: &'b [LogLine],
        filter: &F,
        grouper: &G,
        limit: usize,
    ) where
        T: Display + Hash + Eq + 'b,
        F: Fn(&&LogLine) -> bool,
        G: for<'a> Fn(&'a &'b LogLine) -> T,
    {
        struct Group<'b> {
            lines: Vec<&'b LogLine>,
            bytes: u64,
            key_sum: u64,
        }
        let mut expander = OwningTemplateExpander::new();
        expander
            .set_default("")
            .set("groups-name", groups_name)
            .set("group-key", group_key);
        log_lines.iter()
            .filter(filter)
            .into_group_map_by(grouper)
            .fun(|g| {
                expander.set("groups-count", g.len().to_formatted_string(&Locale::en));
                if g.len() > limit {
                    expander.set("limited", format!("{} most frequent:", limit));
                }
            })
            .into_iter()
            .map(|(value, lines)| {
                let bytes: u64 = lines
                    .iter()
                    .map(|ll| ll.bytes_sent)
                    .sum();
                let key_sum = match self.key {
                    Key::Hits  => lines.len() as u64,
                    Key::Bytes => bytes,
                };
                (value, Group { lines, bytes, key_sum })
            })
            .sorted_by_key(|(_, g)| Reverse(g.key_sum))
            .take(limit)
            .enumerate()
            .for_each(|(idx, (value, g))| {
                let sub = expander.sub("groups");
                sub
                    .set("idx", idx+1)
                    .set("group-value", value)
                    .set_md("hits", self.md_hits(g.lines.len()))
                    .set("percent", to_percent(g.lines.len(), log_lines.len()))
                    .set_md("bytes", self.md_bytes(g.bytes));
            });
        self.print(expander, MD_GROUPS_NO_TRENDS);
    }

    pub fn print_groups_trends<'b, T, F, G>(
        &self,
        groups_name: &str,
        group_key: &str,
        log_lines: &'b [LogLine],
        filter: &F,
        grouper: &G,
        trend_computer: &TrendComputer,
        limit: usize,
        row_idx: bool,
    ) where
        T: Display + Hash + Eq + 'b,
        F: Fn(&&LogLine) -> bool,
        G: for<'a> Fn(&'a &'b LogLine) -> T,
    {
        let mut expander = OwningTemplateExpander::new();
        expander
            .set_default("")
            .set("groups-name", groups_name)
            .set("group-key", group_key);
        log_lines.iter()
            .filter(filter)
            .into_group_map_by(grouper)
            .fun(|g| {
                expander.set("groups-count", g.len().to_formatted_string(&Locale::en));
                if g.len() > limit {
                    expander.set("limited", format!("{} most frequent:", limit));
                }
            })
            .into_iter()
            .map(|(value, lines)| (value, LineGroup::new(lines, trend_computer)))
            .sorted_by_key(|(_, g)| Reverse(g.key_sum))
            .take(limit)
            .enumerate()
            .for_each(|(idx, (value, g))| {
                let sub = expander.sub("groups");
                sub
                    .set("idx", idx+1)
                    .set("group-value", value)
                    .set("histo_line", g.histo_line())
                    .set_md("hits", self.md_hits(g.hits()))
                    .set("percent", to_percent(g.hits(), log_lines.len()))
                    .set_md("bytes", self.md_bytes(g.bytes));
                if g.hits() > 9 {
                    sub.set_md("trend", g.trend.markdown());
                }
            });
        let template = if row_idx {
            MD_GROUPS_TRENDS
        } else {
            MD_GROUPS_TRENDS_NO_ROW_IDX
        };
        self.print(expander, template);
    }

}

fn is_output_piped() -> bool {
    unsafe {
        libc::isatty(libc::STDOUT_FILENO) == 0
    }
}

fn to_percent(count: usize, total: usize) -> String {
    let percent = 100f32 * (count as f32) / (total as f32);
    format!("{:.1}%", percent)
}

