
mod date_filter;
mod ip_filter;
mod status_filter;
mod str_filter;
mod method_filter;

pub use {
    date_filter::*,
    ip_filter::*,
    status_filter::*,
    str_filter::*,
    method_filter::*,
};

use crate::*;

/// apply all filters found in args and print info about
/// the operations
pub fn apply(
    base: &mut LogBase,
    args: &args::Args,
    printer: &md::Printer,
) -> Result<()> {
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
    Ok(())
}

fn filter<F>(
    field_name: &str,
    pattern: &Option<String>,
    retain: F,
    log_base: &mut LogBase,
    printer: &md::Printer,
) -> Result<()>
where
    F: Fn(&mut LogBase, &str) -> Result<()>,
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
