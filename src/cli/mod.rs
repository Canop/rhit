pub mod args;

use {
    crate::*,
    anyhow::*,
    cli_log::*,
    std::path::PathBuf,
};

fn print_analysis(paths: &[PathBuf], args: &args::Args) -> Result<()> {
    let mut log_base = time!("LogBase::new", LogBase::new(paths, args))?;
    let printer = md::Printer::new(args, &log_base);
    let base = &mut log_base;
    let trend_computer = time!("Trend computer initialization", TrendComputer::new(base, args))?;
    md::summary::print_summary(base, &printer);
    time!("Analysis & Printing", md::print_analysis(&log_base, &printer, trend_computer.as_ref()));
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    let args: args::Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("rhit {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mut paths = args.files.clone();
    if paths.is_empty() {
        paths.push(PathBuf::from("/var/log/nginx"));
    }
    if args.lines {
        print_lines(&paths, &args)?;
    } else {
        print_analysis(&paths, &args)?;
    }
    log_mem(Level::Info);
    Ok(())
}

