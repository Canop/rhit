use {
    super::*,
    crate::*,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
};

static MD_NO_TRENDS: &str = r#"
## HTTP status codes:
|:-|:-:|:-:
|**status**|**hits**|**%**
|:-:|-:|-:
${statuses
|${status}|${count}|${percent}
}
|-:
"#;

static MD_TRENDS: &str = r#"
## HTTP status codes:
|:-|:-:|:-:|:-:|:-:
|**status**|**hits**|**%**|**days**|**trend**
|:-:|-:|-:|-:|:-:|
${statuses
|${status}|*${count}*|${percent}|*${histo_line}*|${trend}
}
|-:
"#;

static MD_SHORT: &str = r#"
## HTTP status codes:
|:-:|:-:|:-:|:-:
|**2xx**|**3xx**|**4xx**|**5xx**
|:-:|:-:|:-:|:-:
|*${percent_2xx}*|*${percent_3xx}*|*${percent_4xx}*|*${percent_5xx}*
|-:
"#;

pub fn print_status_codes(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
){
    if printer.detail_level == 0 {
        print_status_summary(log_lines, printer)
    } else if let Some(trend_computer) = trend_computer {
        print_all_status_trends(log_lines, printer, trend_computer)
    } else {
        print_all_status_no_trends(log_lines, printer)
    }
}

fn to_percent(count: usize, total: usize) -> String {
    let percent = 100f32 * (count as f32) / (total as f32);
    format!("{:.1}%", percent)
}

fn print_status_summary(
    log_lines: &[LogLine],
    printer: &Printer,
){
    let mut expander = OwningTemplateExpander::new();
    let (mut s2, mut s3, mut s4, mut s5) = (0, 0, 0, 0);
    for ll in log_lines {
        match ll.status {
            200..=299 => { s2 += 1 }
            300..=399 => { s3 += 1 }
            400..=499 => { s4 += 1 }
            _        => { s5 += 1 }
        }
    }
    expander
        .set("percent_2xx", to_percent(s2, log_lines.len()))
        .set("percent_3xx", to_percent(s3, log_lines.len()))
        .set("percent_4xx", to_percent(s4, log_lines.len()))
        .set("percent_5xx", to_percent(s5, log_lines.len()));
    printer.print(expander, MD_SHORT);
}

fn print_all_status_no_trends(
    log_lines: &[LogLine],
    printer: &Printer,
){
    let mut expander = OwningTemplateExpander::new();
    log_lines.iter()
        .into_group_map_by(|ll| ll.status)
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .for_each(|e| {
            expander.sub("statuses")
                .set("status", e.0)
                .set("count", e.1.len())
                .set("percent", to_percent(e.1.len(), log_lines.len()));
        });
    printer.print(expander, MD_NO_TRENDS);
}

fn print_all_status_trends(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: &TrendComputer,
){
    let mut expander = OwningTemplateExpander::new();
    expander.set_default(" ");
    log_lines.iter()
        .into_group_map_by(|ll| ll.status)
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .map(|(_, lines)| LineGroup::new(lines, trend_computer))
        .for_each(|g| {
            let histo_line = histo_line(
                &g.trend.counts_per_day,
                g.trend.max_day_count(),
                false,
            );
            let sub = expander.sub("statuses");
            sub
                .set("status", g.any().status)
                .set("count", g.lines.len())
                .set("percent", to_percent(g.lines.len(), log_lines.len()))
                .set("histo_line", histo_line);
            if g.lines.len() > 9 {
                sub.set_md("trend", g.trend.markdown());
            }
        });
    printer.print(expander, MD_TRENDS);
}

