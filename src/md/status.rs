use {
    super::*,
    crate::*,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
    termimad::*,
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

pub fn print_status_codes(
    log_lines: &[LogLine],
    skin: &MadSkin,
){
    let mut expander = OwningTemplateExpander::new();
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
    print(expander, MD, skin);
}

