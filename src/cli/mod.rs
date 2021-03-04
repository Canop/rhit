pub mod args;

use {
    crate::*,
    anyhow::*,
    argh,
    std::{
        path::{Path, PathBuf},
    },
};


fn print_analysis(path: &Path, args: &args::Args) -> Result<()> {
    let mut log_base = time!("LogBase::new", LogBase::new(&path, &args))?;
    if log_base.lines.is_empty() {
        eprintln!("no hit in logs");
        return Ok(());
    }
    let printer = md::Printer::new(&args, &log_base);
    let base = &mut log_base;
    let trend_computer = time!("Trend computer initialization", TrendComputer::new(base, &args))?;
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
    let path = args.file.clone().unwrap_or_else(|| PathBuf::from("/var/log/nginx"));
    if args.lines {
        print_lines(&path, &args)?;
    } else {
        print_analysis(&path, &args)?;
    }
    cli_log::log_mem(log::Level::Info);
    Ok(())
}

