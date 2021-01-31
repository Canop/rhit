use {
    super::*,
    crate::*,
    file_size::fit_4,
    have::Fun,
    itertools::*,
    std::{
        cmp::Reverse,
    },
    minimad::OwningTemplateExpander,
};

static MD: &str = r#"
## ${paths-count} distinct paths. ${paths-limit} most popular (excluding resources like images, CSS, JS, etc.):
|:-:|:-|:-:|:-:
|**#**|**path**|**hits**|**resp. size**
|-:|:-|-:|-:|
${popular-paths
|${idx}|${path}|${count}|${bytes}
}
|-:
"#;

pub fn print_popular_paths(
    log_lines: &[LogLine],
    printer: &Printer,
) {
    let mut expander = OwningTemplateExpander::new();
    let n = match printer.detail_level {
        0 => 10,
        1 => 50,
        l => l * 50,
    };
    log_lines
        .iter()
        .filter(|ll| !ll.is_resource())
        .into_group_map_by(|ll| &ll.path)
        .fun(|g| {
            expander
                .set("paths-limit", g.len())
                .set("paths-count", g.len());
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            let sum_bytes: u64 = e.1
                .iter()
                .map(|ll| ll.bytes_sent)
                .sum();
            let bytes = fit_4(sum_bytes / e.1.len() as u64);
            expander.sub("popular-paths")
                .set("idx", idx+1)
                .set("bytes", bytes)
                .set("path", e.0)
                .set("count", e.1.len());
        });
    printer.print(expander, MD);
}

