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
    if let Some(path) = &args.path {
        let mut log_base = LogBase::new(path)?;
        let skin = md::make_skin();
        println!();
        md::print_summary(&log_base, &skin);
        if let Some(pattern) = args.pattern {
            let len_before = log_base.lines.len();
            let regex = Regex::new(&pattern)?;
            log_base.retain_paths_matching(&regex);
            let percent = 100f32 * (log_base.lines.len() as f32) / (len_before as f32);
            let percent = format!("{:.1}%", percent);
            mad_print_inline!(
                &skin,
                "Filtering by pattern *$0* kept **$1** of all lines:\n",
                &pattern,
                &percent,
            );
            md::print_summary(&log_base, &skin);
        }
        md::print_analysis(&log_base, &skin);
    }
    Ok(())
}

