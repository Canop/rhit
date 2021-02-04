use {
    super::*,
    crate::*,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
};

static MD: &str = r#"
## Methods:
|:-|:-:|:-:
|**method**|**hits**|**%**
|:-|-:|-:
${statuses
|${status}|${count}|${percent}
}
|-:
"#;

pub fn print_methods(
    log_lines: &[LogLine],
    printer: &Printer,
){
    let mut expander = OwningTemplateExpander::new();
    log_lines.iter()
        .into_group_map_by(|ll| ll.method)
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

fn to_percent(count: usize, total: usize) -> String {
    let percent = 100f32 * (count as f32) / (total as f32);
    format!("{:.2}%", percent)
}


