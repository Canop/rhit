use {
    crate::*,
    crossterm::style::{Attribute::*, Color::*},
    have::Fun,
    itertools::*,
    std::{
        cmp::Reverse,
    },
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::*,
};

static SUMMARY_MD: &str = r#"
**${hits-count}** hits in *${days}* days from *${start}* to *${end}*
"#;

static MAIN_MD: &str = r#"
## HTTP status codes:
|:-|:-:|:-:
|**status**|**hits**|**%**
|:-|-:|-:
${statuses
|${status}|${count}|${percent}
}
|-:
## ${remote-addr-count} distinct remote adresses. ${remote-addr-limit} most used:
|:-|:-:
|**remote address**|**hits**
|:-|-:
${popular-remote-addresses
|${remote-address}|${count}
}
|-:
## ${referers-count} referers. ${referers-limit} most frequent:
|:-|:-:
|**referer**|**hits**
|:-|-:
${popular-referers
|${referer}|${count}
}
|-:
## ${paths-count} distinct paths. most popular (excluding resources like images, CSS, JS, etc.):
|:-:|:-|:-:|:-:
|**#**|**path**|**hits**|**usual bytes per resp.**
|-:|:-|-:|-:|
${popular-paths
|${idx}|${path}|${count}|${bytes}
}
|-:
"#;

pub fn print_summary(log_base: &LogBase, skin: &MadSkin) {
    let mut expander = OwningTemplateExpander::new();
    fill_summary(&mut expander, log_base);
    print(expander, SUMMARY_MD, skin);
}

pub fn print(expander: OwningTemplateExpander, template: &str, skin: &MadSkin) {
    let (width, _) = terminal_size();
    let template = TextTemplate::from(template);
    let text = expander.expand(&template);
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
}

pub fn print_analysis(log_base: &LogBase, skin: &MadSkin) {
    let mut expander = OwningTemplateExpander::new();
    let log_lines = &log_base.lines;
    fill_status_codes(&mut expander, log_lines);
    fill_popular_remote_addresses(&mut expander, log_lines, 5);
    fill_popular_referers(&mut expander, log_lines, 100);
    fill_popular_paths(&mut expander, log_lines, 100);
    print(expander, MAIN_MD, skin);
}

fn fill_summary(expander: &mut OwningTemplateExpander, log_base: &LogBase) {
    expander
        .set("hits-count", log_base.lines.len())
        .set("days", (log_base.end_time()-log_base.start_time()).num_days())
        .set("start", log_base.start_time())
        .set("end", log_base.end_time());
}

fn fill_status_codes(expander: &mut OwningTemplateExpander, log_lines: &[LogLine]) {
    log_lines.iter()
        .into_group_map_by(|ll| ll.status)
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .for_each(|e| {
            let count = e.1.len();
            let percent = 100f32 * (count as f32) / (log_lines.len() as f32);
            expander.sub("statuses")
                .set("status", e.0)
                .set("count", count)
                .set("percent", format!("{:.1}%", percent));
        });
}

fn fill_popular_remote_addresses(expander: &mut OwningTemplateExpander, log_lines: &[LogLine], n: usize) {
    log_lines.iter()
        .into_group_map_by(|ll| ll.remote_addr)
        .fun(|g| {
            expander
                .set("remote-addr-count", g.len())
                .set("remote-addr-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .for_each(|e| {
            expander.sub("popular-remote-addresses")
                .set("remote-address", e.0)
                .set("count", e.1.len());
        });
}

fn fill_popular_referers(expander: &mut OwningTemplateExpander, log_lines: &[LogLine], n: usize) {
    log_lines.iter()
        .filter(|ll| ll.referer.len()>1)
        .into_group_map_by(|ll| &ll.referer)
        .fun(|g| {
            expander
                .set("referers-count", g.len())
                .set("referers-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .for_each(|e| {
            expander.sub("popular-referers")
                .set("referer", e.0)
                .set("count", e.1.len());
        });
}

fn fill_popular_paths(expander: &mut OwningTemplateExpander, log_lines: &[LogLine], n: usize) {
    log_lines
        .iter()
        .filter(|ll| !ll.is_resource())
        .into_group_map_by(|ll| &ll.path)
        .fun(|g| {
            expander.set("paths-count", g.len());
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            // to display the usual hits, we'll first take
            // the most frequent ones until we get 85% of them
            let total_hits = e.1.len();
            let mut sum_hits = 0;
            let mut sizes: Vec<u64> = e.1
                .iter()
                .into_group_map_by(|ll| &ll.bytes_sent)
                .into_iter()
                .map(|e| (e.0, e.1.len()))
                .sorted_by_key(|e| Reverse(e.1))
                .take_while(|e| {
                    let take = sum_hits * 100 <= total_hits * 85;
                    sum_hits += e.1;
                    take
                })
                .map(|i| *i.0)
                .collect();
            sizes.sort_unstable();
            let sizes: Vec<String> = sizes
                .drain(..)
                .map(file_size::fit_4)
                .unique()
                .collect();
            let bytes = match sizes.len() {
                0 => "???".to_string(),
                1 => sizes[0].clone(),
                2|3|4 => sizes.join(", "),
                _ => format!("{} to {}", sizes[0], sizes[sizes.len()-1]),
            };
            expander.sub("popular-paths")
                .set("idx", idx+1)
                .set("bytes", bytes)
                .set("path", e.0)
                .set("count", e.1.len());
        });
}


pub fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[1].compound_style.remove_attr(Underlined);
    skin.italic.remove_attr(Italic);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}
