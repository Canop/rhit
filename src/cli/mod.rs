pub mod args;
mod help;

use {
    crate::*,
    args::Args,
    clap::Parser,
    cli_log::*,
    std::path::PathBuf,
};

const DEFAULT_NGINX_LOCATION: &str = "/var/log/nginx";

fn print_analysis(paths: &[PathBuf], args: &args::Args) -> Result<(), RhitError> {
    let mut log_base = time!("LogBase::new", LogBase::new(paths, args))?;
    let printer = md::Printer::new(args, &log_base);
    let base = &mut log_base;
    let trend_computer = time!("Trend computer initialization", TrendComputer::new(base, args))?;
    md::summary::print_summary(base, &printer);
    time!("Analysis & Printing", md::print_analysis(&log_base, &printer, trend_computer.as_ref()));
    Ok(())
}

pub fn run() -> Result<(), RhitError> {
    let args = Args::parse();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.help {
        help::print();
        return Ok(());
    }
    let mut paths = args.files.clone();
    if paths.is_empty() {
        paths.push(PathBuf::from(DEFAULT_NGINX_LOCATION));
    }
    let result = match args.output {
        Output::Raw => print_raw_lines(&paths, &args),
        Output::Tables => print_analysis(&paths, &args),
        Output::Csv => print_csv_lines(&paths, &args),
        Output::Json => print_json_lines(&paths, &args),
    };
    if let Err(RhitError::PathNotFound(ref path)) = result {
        if path == &PathBuf::from(DEFAULT_NGINX_LOCATION) {
            eprintln!(
                "No nginx log found at default location, do you have nginx set up?"
            );
        }
    }
    log_mem(Level::Info);
    result
}

