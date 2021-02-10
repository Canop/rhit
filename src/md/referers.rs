use {
    super::*,
    crate::*,
    have::Fun,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
    num_format::{Locale, ToFormattedString},
};

static MD: &str = r#"
## ${referers-count} referers. ${referers-limit} most frequent:
|:-:|:-|:-:
|**#**|**referer**|**hits**
|-:|:-|-:
${popular-referers
|${idx}|${referer}|*${count}*
}
|-:
"#;

pub fn print_popular_referers(
    log_lines: &[LogLine],
    printer: &Printer,
) {
    let mut expander = OwningTemplateExpander::new();
    let n = match printer.detail_level {
        0 => 5,
        1 => 10,
        l => l * 20,
    };
    log_lines.iter()
        .filter(|ll| ll.referer.len()>1)
        .into_group_map_by(|ll| &ll.referer)
        .fun(|g| {
            expander
                .set("referers-count", g.len().to_formatted_string(&Locale::en))
                .set("referers-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            expander.sub("popular-referers")
                .set("idx", idx+1)
                .set("referer", e.0)
                .set("count", e.1.len().to_formatted_string(&Locale::en));
        });
    printer.print(expander, MD);
}

