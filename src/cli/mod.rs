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
    let printer = md::Printer::new(&args);
    let path = args.file.unwrap_or_else(|| PathBuf::from("/var/log/nginx"));
    let mut log_base = LogBase::new(&path)?;
    if log_base.lines.is_empty() {
        eprintln!("no hit in logs");
        return Ok(());
    }
    // the trend computer needs the whole unfiltered base for initialization
    let trend_computer = TrendComputer::new(&log_base);
    md::summary::print_summary(&log_base, &printer);
    if let Some(pattern) = &args.status {
        let len_before = log_base.lines.len();
        log_base.retain_status_matching(pattern)?;
        if after_filter("status", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.method {
        let len_before = log_base.lines.len();
        log_base.retain_methods_matching(pattern)?;
        if after_filter("method", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.addr {
        let len_before = log_base.lines.len();
        log_base.retain_remote_addr_matching(pattern)?;
        if after_filter("remote address", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.date {
        let len_before = log_base.lines.len();
        log_base.retain_dates_matching(pattern)?;
        if after_filter("date", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.path {
        let len_before = log_base.lines.len();
        log_base.retain_paths_matching(pattern)?;
        if after_filter("path", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    if let Some(pattern) = &args.referer {
        let len_before = log_base.lines.len();
        log_base.retain_referers_matching(pattern)?;
        if after_filter("referer", pattern, len_before, &log_base, &printer)? {
            return Ok(());
        }
    }
    md::print_analysis(&log_base, &printer, trend_computer.as_ref());
    Ok(())
}

fn after_filter(
    field: &str,
    pattern: &str,
    before: usize,
    log_base: &LogBase,
    printer: &md::Printer,
) -> Result<bool> {
    let after = log_base.lines.len();
    let percent = 100f32 * (after as f32) / (before as f32);
    let percent = format!("{:.2}%", percent);
    mad_print_inline!(
        &printer.skin,
        "Filtering by *$0* on pattern **$1** kept **$2** of previous lines:\n",
        field,
        &pattern,
        &percent,
    );
    if log_base.lines.is_empty() {
        println!("nothing to display");
        return Ok(true);
    }
    md::summary::print_summary(&log_base, &printer);
    Ok(false)
}
