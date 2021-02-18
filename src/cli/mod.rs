pub mod args;

use {
    crate::*,
    argh,
};

pub fn run() -> anyhow::Result<()> {
    let args: args::Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("rhit {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let path = args.file.clone().unwrap_or_else(|| PathBuf::from("/var/log/nginx"));
    let mut log_base = LogBase::new(&path)?;
    if log_base.lines.is_empty() {
        eprintln!("no hit in logs");
        return Ok(());
    }
    let printer = md::Printer::new(&args, &log_base);
    let base = &mut log_base;
    // the trend computer needs the whole unfiltered base for initialization
    // and thus needs to be built before filtering
    let trend_computer = TrendComputer::new(base, &args)?;
    md::summary::print_summary(base, &printer);
    filter(
        "status", &args.status, LogBase::retain_status_matching,
        base, &printer,
    )?;
    filter(
        "method", &args.method, LogBase::retain_methods_matching,
        base, &printer,
    )?;
    filter(
        "date", &args.date, LogBase::retain_dates_matching,
        base, &printer,
    )?;
    filter(
        "remote IP address", &args.ip, LogBase::retain_remote_addr_matching,
        base, &printer,
    )?;
    filter(
        "path", &args.path, LogBase::retain_paths_matching,
        base, &printer,
    )?;
    filter(
        "referer", &args.referer, LogBase::retain_referers_matching,
        base, &printer,
    )?;

    md::print_analysis(&log_base, &printer, trend_computer.as_ref());
    Ok(())
}


fn filter<F>(
    field_name: &str,
    pattern: &Option<String>,
    retain: F,
    log_base: &mut LogBase,
    printer: &md::Printer,
) -> Result<()>
where F: Fn(&mut LogBase, &str) -> Result<()>,
{
    let before = log_base.lines.len();
    if before > 0 {
        if let Some(pattern) = pattern {
            retain(log_base, pattern)?;
            let after = log_base.lines.len();
            let percent = 100f32 * (after as f32) / (before as f32);
            let percent = format!("{:.2}%", percent);
            mad_print_inline!(
                &printer.skin,
                "Filtering by $0 on pattern `$1` kept **$2** of previous lines:\n",
                field_name,
                &pattern,
                &percent,
            );
            if log_base.lines.is_empty() {
                println!("nothing to display");
            } else {
                md::summary::print_summary(&log_base, &printer);
            }
        }
    }
    Ok(())
}
