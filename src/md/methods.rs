use {
    super::*,
    crate::*,
    itertools::*,
    minimad::OwningTemplateExpander,
    num_format::{Locale, ToFormattedString},
    std::cmp::Reverse,
};

static MD_NO_TRENDS: &str = r#"
## Methods:
|:-|:-:|:-:
|**method**|**hits**|**%**
|:-:|-:|-:
${methods
|${method}|*${count}*|${percent}
}
|-:
"#;

static MD_TRENDS: &str = r#"
## Methods:
|:-|:-:|:-:|:-:|:-:
|**method**|**hits**|**%**|**days**|**trend**
|:-:|-:|-:|-:|:-:|
${methods
|${method}|*${count}*|${percent}|*${histo_line}*|${trend}
}
|-:
"#;

pub fn print_methods(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
){
    if let Some(trend_computer) = trend_computer {
        print_methods_trends(log_lines, printer, trend_computer)
    } else {
        print_methods_no_trends(log_lines, printer)
    }
}

fn print_methods_no_trends(
    log_lines: &[LogLine],
    printer: &Printer,
){
    let mut expander = OwningTemplateExpander::new();
    log_lines.iter()
        .into_group_map_by(|ll| ll.method)
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .for_each(|e| {
            expander.sub("methods")
                .set("method", e.0)
                .set("count", e.1.len().to_formatted_string(&Locale::en))
                .set("percent", to_percent(e.1.len(), log_lines.len()));
        });
    printer.print(expander, MD_NO_TRENDS);
}

fn print_methods_trends(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: &TrendComputer,
){
    let mut expander = OwningTemplateExpander::new();
    expander.set_default(" ");
    log_lines.iter()
        .into_group_map_by(|ll| ll.method)
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .map(|(_, lines)| LineGroup::new(lines, trend_computer))
        .for_each(|g| {
            let histo_line = histo_line(
                &g.trend.counts_per_day,
                g.trend.max_day_count(),
                false,
            );
            let sub = expander.sub("methods");
            sub
                .set("method", g.any().method)
                .set("count", g.hits().to_formatted_string(&Locale::en))
                .set("percent", to_percent(g.hits(), log_lines.len()))
                .set("histo_line", histo_line);
            if g.hits() > 9 {
                sub.set_md("trend", g.trend.markdown());
            }
        });
    printer.print(expander, MD_TRENDS);
}

fn to_percent(count: usize, total: usize) -> String {
    let percent = 100f32 * (count as f32) / (total as f32);
    format!("{:.2}%", percent)
}


