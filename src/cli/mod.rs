mod args;

pub use args::*;

use {
    crate::*,
    argh,
};

pub fn run() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("rhit {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let path = args.file.unwrap_or_else(|| PathBuf::from("/var/log/nginx"));
    let mut log_base = LogBase::new(&path)?;
    let skin = md::make_skin();
    if log_base.lines.is_empty() {
        println!("no hit in logs");
        return Ok(());
    }
    md::print_summary(&log_base, &skin);
    if let Some(pattern) = &args.path {
        let len_before = log_base.lines.len();
        log_base.retain_paths_matching(&Regex::new(pattern)?);
        if after_filter("path", pattern, len_before, &log_base, &skin)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.referer {
        let len_before = log_base.lines.len();
        log_base.retain_referers_matching(&Regex::new(pattern)?);
        if after_filter("referer", pattern, len_before, &log_base, &skin)? {
            return Ok(());
        }
    }
    let histogram = Histogram::of_days(&log_base);
    histogram.print(&skin);
    md::print_analysis(&log_base, &skin);
    Ok(())
}

fn after_filter(
    field: &str,
    pattern: &str,
    before: usize,
    log_base: &LogBase,
    skin: &MadSkin,
) -> Result<bool> {
    let after = log_base.lines.len();
    let percent = 100f32 * (after as f32) / (before as f32);
    let percent = format!("{:.2}%", percent);
    mad_print_inline!(
        &skin,
        "Filtering by *$0* on pattern **$1** kept **$2** of previous lines:\n",
        field,
        &pattern,
        &percent,
    );
    if log_base.lines.is_empty() {
        println!("nothing to display");
        return Ok(true);
    }
    md::print_summary(&log_base, &skin);
    Ok(false)
}
