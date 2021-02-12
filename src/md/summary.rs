use {
    super::*,
    crate::*,
    minimad::OwningTemplateExpander,
};

static SUMMARY_MD: &str = r#"
${hits} hits and ${bytes} from **${start}** to **${end}**
"#;

pub fn print_summary(log_base: &LogBase, printer: &Printer) {
    let mut expander = OwningTemplateExpander::new();
    let bytes = log_base.lines
        .iter()
        .map(|line| line.bytes_sent)
        .sum();
    expander
        .set_md("hits", printer.md_hits(log_base.lines.len()))
        .set_md("bytes", printer.md_bytes(bytes))
        .set("start", log_base.start_time())
        .set("end", log_base.end_time());
    printer.print(expander, SUMMARY_MD);
}


