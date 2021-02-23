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
    let mut log_base = time!("LogBase::new", LogBase::new(&path, !args.no_name_check))?;
    if log_base.lines.is_empty() {
        eprintln!("no hit in logs");
        return Ok(());
    }
    let printer = md::Printer::new(&args, &log_base);
    let base = &mut log_base;
    // the trend computer needs the whole unfiltered base for initialization
    // and thus needs to be built before filtering
    let trend_computer = time!("Trend computer initialization", TrendComputer::new(base, &args))?;
    md::summary::print_summary(base, &printer);
    time!("Filtering", filters::apply(base, &args, &printer)?);
    time!("Analysis & Printing", md::print_analysis(&log_base, &printer, trend_computer.as_ref()));
    Ok(())
}

