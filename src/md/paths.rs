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

const DEBUG_TRENDS: bool = false;

static MD_TITLE: &str = r#"
## ${count} distinct paths (excluding resources like images, CSS, JS, etc.)
"#;

static MD_NO_TRENDS: &str = r#"
|:-:|:-|:-:|:-:
|**#**|**path**|**hits**|**resp. size**
|-:|:-|-:|-:|
${paths
|${idx}|${path}|${count}|${bytes}
}
|-:
"#;

static MD_TRENDS_DEBUG: &str = r#"
### ${title}:
|:-:|:-|:-:|:-:|:-:|:-:|:-:|:-:
|**#**|**path**|**hits**|**resp. size**|**days**|**previous ${ref_size} days**|**last ${tail_size} days**|**trend**
|-:|:-|-:|-:|:-:|-:|-:|:-:|
${paths
|${idx}|${path}|*${count}*|${bytes}|*${histo_line}*|${ref_count}|${tail_count}|${trend}
}
|-:
"#;

static MD_TRENDS: &str = r#"
### ${title}:
|:-:|:-|:-:|:-:|:-:|:-:
|**#**|**path**|**hits**|**resp. size**|**days**|**trend**
|-:|:-|-:|-:|-:|:-:|
${paths
|${idx}|${path}|*${count}*|${bytes}|*${histo_line}*|${trend}
}
|-:
"#;

pub fn print_paths_no_trends(
    base: &LogBase,
    printer: &Printer,
) {
    let n = match printer.detail_level {
        0 => 10,
        1 => 50,
        l => l * 50,
    };
    let mut expander = OwningTemplateExpander::new();
    base
        .lines
        .iter()
        .filter(|ll| !ll.is_resource())
        .into_group_map_by(|ll| &ll.path)
        .fun(|g| {
            let mut expander = OwningTemplateExpander::new();
            expander.set("count", g.len());
            printer.print(expander, MD_TITLE);
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
            let sub = expander.sub("paths");
            sub
                .set("idx", idx+1)
                .set("bytes", bytes)
                .set("path", e.0)
                .set("count", e.1.len());
        });
    printer.print(expander, MD_NO_TRENDS);
}

pub fn print_paths(
    base: &LogBase,
    printer: &Printer,
    trend_computer: &TrendComputer,
    popular: bool,
    trendy: bool,
) {
    if !(trendy || popular) {
        return;
    }

    let groups: Vec<LineGroup> = base
        .lines
        .iter()
        .filter(|ll| !ll.is_resource())
        .into_group_map_by(|ll| &ll.path)
        .into_iter()
        .map(|(_, lines)| LineGroup::new(lines, trend_computer))
        .collect();

    let mut title_expander = OwningTemplateExpander::new();
    title_expander.set("count", groups.len());
    printer.print(title_expander, MD_TITLE);

    if popular {
        let n = match printer.detail_level {
            0 => 10,
            l => l * 50,
        };
        let popular_paths = groups
            .iter()
            .sorted_by_key(|g| Reverse(g.lines.len()))
            .take(n);
        print_table_with_trends("Most popular paths", popular_paths, printer);
    }
    if trendy {
        let n = match printer.detail_level {
            0 => 5,
            l => l * 10,
        };
        let treshold = (base.lines.len() / 10000)
            .max(5)
            .min(30);
        let trendy_paths = groups
            .iter()
            .filter(|g| g.lines.len() >= treshold)
            .sorted_by_key(|g| Reverse(&g.trend))
            .take(n);
        print_table_with_trends("More popular paths", trendy_paths, printer);
        let trendy_paths = groups
            .iter()
            .filter(|g| g.lines.len() >= treshold)
            .sorted_by_key(|g| &g.trend)
            .take(n);
        print_table_with_trends("Less popular paths", trendy_paths, printer);
    }
}

fn print_table_with_trends(
    title: &str,
    groups: std::iter::Take<std::vec::IntoIter<&line_group::LineGroup<'_>>>,
    printer: &Printer,
) {
    let mut expander = OwningTemplateExpander::new();
    expander
        .set("title", title);
    groups
        .enumerate()
        .for_each(|(idx, g)| {
            let sum_bytes: u64 = g.lines
                .iter()
                .map(|ll| ll.bytes_sent)
                .sum();
            let bytes = fit_4(sum_bytes / g.lines.len() as u64);
            let sub = expander.sub("paths");
            let histo_line = histo_line(
                &g.trend.counts_per_day,
                g.trend.max_day_count(),
                false,
            );
            sub
                .set("idx", idx+1)
                .set("bytes", bytes)
                .set("path", &g.any().path)
                .set("count", g.lines.len())
                .set("histo_line", histo_line)
                .set("ref_count", g.trend.ref_count)
                .set("tail_count", g.trend.tail_count)
                .set_md("trend", g.trend.markdown());
        });
    printer.print(expander, if DEBUG_TRENDS { MD_TRENDS_DEBUG } else { MD_TRENDS });
}