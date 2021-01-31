use {
    super::*,
    crate::*,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
};

static MD: &str = r#"
## HTTP status codes:
|:-|:-:|:-:
|**status**|**hits**|**%**
|:-|-:|-:
${statuses
|${status}|${count}|${percent}
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
){
    if printer.detail_level == 0 {
        print_status_summary(log_lines, printer)
    } else {
        print_all_status(log_lines, printer)
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
pub fn print_all_status(
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
    printer.print(expander, MD);
}

