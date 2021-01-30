use {
    super::*,
    crate::*,
    have::Fun,
    itertools::*,
    std::{
        cmp::Reverse,
    },
    minimad::OwningTemplateExpander,
    termimad::*,
};

static MD: &str = r#"
## ${paths-count} distinct paths. ${paths-limit} most popular (excluding resources like images, CSS, JS, etc.):
|:-:|:-|:-:|:-:
|**#**|**path**|**hits**|**usual bytes per resp.**
|-:|:-|-:|-:|
${popular-paths
|${idx}|${path}|${count}|${bytes}
}
|-:
"#;

pub fn print_popular_paths(
    log_lines: &[LogLine],
    detail_level: usize,
    skin: &MadSkin,
) {
    let mut expander = OwningTemplateExpander::new();
    let n = match detail_level {
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
    print(expander, MD, skin);
}

