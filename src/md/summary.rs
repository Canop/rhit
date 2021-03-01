use {
    super::*,
    crate::*,
    minimad::OwningTemplateExpander,
};

static SUMMARY_MD: &str = r#"
${hits} hits and ${bytes} from **${start}** to **${end}**
${filterings
Filtering by ${field} on pattern `${pattern}` removed **${removed_percent}** of total lines
}
${filtered-stats
 **==>** ${hits} hits and ${bytes}
}
"#;

pub fn print_summary(base: &LogBase, printer: &Printer) {
    let mut expander = OwningTemplateExpander::new();
    let total_bytes = base.unfiltered_histogram.total_bytes_sent();
    expander
        .set_md("hits", printer.md_hits(base.unfiltered_count as usize))
        .set_md("bytes", printer.md_bytes(total_bytes))
        .set("start", base.start_time())
        .set("end", base.end_time());
    if base.filterer.has_filters() {
        let total_hits = base.unfiltered_count as f32;
        for filtering in &base.filterer.filterings {
            let removed = filtering.removed_count as f32;
            let percent = format!("{:.2}%", 100f32 * removed / total_hits);
            expander.sub("filterings")
                .set("field", filtering.filter.field_name())
                .set("pattern", &filtering.pattern)
                .set("removed_percent", percent);
        }
        let filtered_bytes = base.filtered_histogram.total_bytes_sent();
        expander.sub("filtered-stats")
            .set_md("hits", printer.md_hits(base.filtered_count as usize))
            .set_md("bytes", printer.md_bytes(filtered_bytes));
    }
    printer.print(expander, SUMMARY_MD);
}


