use {
    super::*,
    crate::*,
    minimad::OwningTemplateExpander,
};

static SUMMARY_MD: &str = r#"
**${hits-count}** hits from *${start}* to *${end}*
"#;

pub fn print_summary(log_base: &LogBase, printer: &Printer) {
    let mut expander = OwningTemplateExpander::new();
    fill_summary(&mut expander, log_base);
    printer.print(expander, SUMMARY_MD);
}

fn fill_summary(expander: &mut OwningTemplateExpander, log_base: &LogBase) {
    expander
        .set("hits-count", log_base.lines.len())
        .set("start", log_base.start_time())
        .set("end", log_base.end_time());
}

