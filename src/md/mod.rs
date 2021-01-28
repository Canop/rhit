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
## ${paths-count} distinct paths. ${paths-limit} most popular (excluding resources like images, CSS, JS, etc.):
|:-:|:-|:-:
|**#**|**path**|**hits**
|-:|:-|-:
${popular-paths
|${idx}|${path}|${count}
}
|-:
## ${downloads-count} "download" like paths. ${downloads-limit} most popular:
|:-:|-:|:-|:-:
|**#**|**bytes**|**path**|**hits**
|-:|-:|:-|-:
${popular-downloads
|${idx}|${bytes}|${path}|${count}
}
|-:
"#;

pub fn print_summary(log_base: &LogBase, skin: &MadSkin) {
    let mut expander = OwningTemplateExpander::new();
    fill_summary(&mut expander, log_base);
    let (width, _) = terminal_size();
    let template = TextTemplate::from(SUMMARY_MD);
    let text = expander.expand(&template);
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
}

pub fn print_analysis(log_base: &LogBase, skin: &MadSkin) {
    let mut expander = OwningTemplateExpander::new();
    fill_summary(&mut expander, log_base);
    let log_lines = &log_base.lines;
    fill_status_codes(&mut expander, log_lines);
    fill_popular_remote_addresses(&mut expander, log_lines, 5);
    fill_popular_paths(&mut expander, log_lines, 100);
    fill_popular_downloads(&mut expander, log_lines, 50);
    let (width, _) = terminal_size();
    let template = TextTemplate::from(MAIN_MD);
    let text = expander.expand(&template);
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
}

fn fill_summary(expander: &mut OwningTemplateExpander, log_base: &LogBase) {
    expander
        .set("hits-count", log_base.lines.len())
        .set("days", (log_base.end_time()-log_base.start_time()).num_days())
        .set("start", log_base.start_time())
        .set("end", log_base.end_time());
}

fn fill_status_codes(expander: &mut OwningTemplateExpander, log_lines: &Vec<LogLine>) {
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

fn fill_popular_remote_addresses(expander: &mut OwningTemplateExpander, log_lines: &Vec<LogLine>, n: usize) {
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

fn fill_popular_paths(expander: &mut OwningTemplateExpander, log_lines: &Vec<LogLine>, n: usize) {
    log_lines
        .iter()
        .filter(|ll| !ll.is_resource())
        .into_group_map_by(|ll| &ll.path)
        .fun(|g| {
            expander
                .set("paths-count", g.len())
                .set("paths-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            expander.sub("popular-paths")
                .set("idx", idx+1)
                .set("path", e.0)
                .set("count", e.1.len());
        });
}

fn fill_popular_downloads(expander: &mut OwningTemplateExpander, log_lines: &Vec<LogLine>, n: usize) {
    log_lines
        .iter()
        .filter(|ll| ll.looks_like_download())
        .into_group_map_by(|ll| &ll.path)
        .fun(|g| {
            expander
                .set("downloads-count", g.len())
                .set("downloads-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            let mut sizes: Vec<u64> = e.1
                .iter()
                .into_group_map_by(|ll| &ll.bytes_sent)
                .drain()
                .map(|i| *i.0)
                .collect();
            sizes.sort();
            let sizes: Vec<String> = sizes
                .drain(..)
                .map(file_size::fit_4)
                .collect();
            let bytes = match sizes.len() {
                1 => sizes[0].clone(),
                2|3 => sizes.join(", "),
                //n => format!("{} sizes from {} to {}", n, sizes[0], sizes[sizes.len()-1]),
                _ => format!("from {} to {}", sizes[0], sizes[sizes.len()-1]),
            };
            expander.sub("popular-downloads")
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
