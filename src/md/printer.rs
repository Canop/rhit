use {
    super::*,
    crate::*,
    have::Fun,
    itertools::*,
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

static MD_GROUPS_TRENDS_NO_ROW_IDX: &str = r#"
## ${title}
|:-|:-:|:-:|:-:|:-:|:-:
|**${group-key}**|**hits**|**%**|**bytes**|**days**|**trend**
|:-:|-:|-:|-:|-:|:-:|
${groups
|${group-value}|${hits}|${percent}|${bytes}|*${histo-line}*|${trend}
}
|-:
"#;

static MD_GROUPS_TRENDS: &str = r#"
## ${title}
|:-:|:-|:-:|:-:|:-:|:-:
|**#**|**${group-key}**|**hits**|**bytes**|**days**|**trend**
|-:|:-|-:|-:|-:|:-:|
${groups
|${idx}|${group-value}|${hits}|${bytes}|*${histo-line}*|${trend}
}
|-:
"#;

pub struct Printer {
    pub skin: MadSkin,
    pub fields: Fields,
    pub terminal_width: usize,
    pub detail_level: usize,
    pub key: Key,
    pub date_filter: Option<DateFilter>,
    pub changes: bool,
    pub all_paths: bool,
}

impl Printer {
    pub fn new(args: &args::Args, log_base: &LogBase) -> Self {
        let detail_level = args.length;
        let fields = args.fields.clone();
        let terminal_width = terminal_size().0 as usize;
        let color = args.color.value().unwrap_or(!is_output_piped());
        let skin = skin::make_skin(color);
        let key = args.key;
        let date_filter = log_base.filterer.date_filter().map(|f| f.clone());
        let changes = args.changes;
        let all_paths = args.all;
        Self {
            skin,
            fields,
            terminal_width,
            detail_level,
            key,
            date_filter,
            changes,
            all_paths,
        }
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
        section: &Section,
        log_lines: &'b [LogLine],
        filter: F,
        grouper: G,
        trend_computer: Option<&TrendComputer>,
    ) where
        T: Display + Hash + Eq + 'b,
        F: Fn(&&LogLine) -> bool,
        G: for<'a> Fn(&'a &'b LogLine) -> T,
    {
        if let Some(trend_computer) = trend_computer {
            self.print_groups_trends(
                section,
                log_lines,
                filter,
                grouper,
                trend_computer,
            );
        } else {
            self.print_groups_no_trends(
                section,
                log_lines,
                filter,
                grouper,
            );
        }
    }

    pub fn print_groups_no_trends<'b, T, F, G>(
        &self,
        section: &Section,
        log_lines: &'b [LogLine],
        filter: F,
        grouper: G,
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
            .set("groups-name", section.groups_name)
            .set("group-key", section.group_key);
        log_lines
            .iter()
            .filter(filter)
            .into_group_map_by(grouper)
            .fun(|g| {
                expander.set("groups-count", g.len().to_formatted_string(&Locale::en));
                if let View::Limited(limit) = section.view {
                    if g.len() > limit {
                        expander.set("limited", format!("{} most frequent:", limit));
                    }
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
            .sorted_unstable_by_key(|(_, g)| Reverse(g.key_sum))
            .take(section.view.limit())
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
        section: &Section,
        log_lines: &'b [LogLine],
        filter: F,
        grouper: G,
        trend_computer: &TrendComputer,
    ) where
        T: Display + Hash + Eq + 'b,
        F: Fn(&&LogLine) -> bool,
        G: for<'a> Fn(&'a &'b LogLine) -> T,
    {
        let groups: Vec<LineGroup<T>> = log_lines
            .iter()
            .filter(filter)
            .into_group_map_by(grouper)
            .into_iter()
            .map(|(value, lines)| LineGroup::new(value, lines, trend_computer))
            .collect();
        let title = match section.view {
            View::Full => section.groups_name.to_owned(),
            View::Limited(limit) => {
                let mut title = format!(
                    "{} {}",
                    groups.len().to_formatted_string(&Locale::en),
                    section.groups_name,
                );
                if groups.len() > limit {
                    title.push_str(&format!(". {} most frequent:", limit));
                }
                title
            }
        };
        let popular_groups = groups
            .iter()
            .sorted_unstable_by_key(|g| Reverse(g.key_sum))
            .take(section.view.limit());
        self.print_table_with_trends(
            &title,
            section,
            popular_groups,
            log_lines.len(),
        );
        if self.changes && section.changes && section.view.limit() < log_lines.len() {
            let limit = match self.detail_level {
                0 => 5,
                l => l * 10,
            };
            let more_popular_groups = groups
                .iter()
                .filter(|g| g.lines.len() > 9)
                .sorted_unstable_by_key(|g| Reverse(&g.trend))
                .take(limit);
            let title = format!("More popular {}", section.groups_name);
            self.print_table_with_trends(
                &title,
                section,
                more_popular_groups,
                log_lines.len(),
            );
            let less_popular_groups = groups
                .iter()
                .filter(|g| g.lines.len() > 9)
                .sorted_unstable_by_key(|g| &g.trend)
                .take(limit);
            let title = format!("Less popular {}", section.groups_name);
            self.print_table_with_trends(
                &title,
                section,
                less_popular_groups,
                log_lines.len(),
            );
        }
    }

    pub fn print_table_with_trends<'b, T, I>(
        &self,
        title: &str,
        section: &Section,
        groups: I,
        total_count: usize,
    ) where
        T: Display + Hash + Eq + 'b,
        I: Iterator<Item = &'b LineGroup<'b, T>>,
    {
        let mut rows_count = 0;
        let mut expander = OwningTemplateExpander::new();
        expander
            .set_default("")
            .set("group-key", section.group_key)
            .set("title", title);
        groups.enumerate().for_each(|(idx, g)| {
            rows_count += 1;
            let sub = expander.sub("groups");
            sub.set("idx", idx + 1)
                .set("group-value", &g.value)
                .set_md("hits", self.md_hits(g.hits()))
                .set_md("bytes", self.md_bytes(g.bytes))
                .set("histo-line", g.histo_line())
                .set("ref_count", g.trend.ref_count)
                .set("tail_count", g.trend.tail_count);
            if matches!(section.view, View::Full) {
                sub.set("percent", to_percent(g.lines.len(), total_count));
            }
            if g.hits() > 9 {
                sub.set_md("trend", g.trend.markdown());
            }
        });
        if rows_count == 0 {
            println!("{} : none", title);
        } else {
            let template = match section.view {
                View::Full => MD_GROUPS_TRENDS_NO_ROW_IDX,
                View::Limited(_) => MD_GROUPS_TRENDS,
            };
            self.print(expander, template);
        }
    }
}

fn is_output_piped() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) == 0 }
}

fn to_percent(count: usize, total: usize) -> String {
    let percent = 100f32 * (count as f32) / (total as f32);
    format!("{:.1}%", percent)
}

