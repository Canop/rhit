use {
    super::*,
    crate::*,
    minimad::OwningTemplateExpander,
    termimad::*,
};

static SUMMARY_MD: &str = r#"
**${hits-count}** hits from *${start}* to *${end}*
"#;

pub fn print_summary(log_base: &LogBase, skin: &MadSkin) {
    let mut expander = OwningTemplateExpander::new();
    fill_summary(&mut expander, log_base);
    print(expander, SUMMARY_MD, skin);
}

fn fill_summary(expander: &mut OwningTemplateExpander, log_base: &LogBase) {
    expander
        .set("hits-count", log_base.lines.len())
        .set("start", log_base.start_time())
        .set("end", log_base.end_time());
}

