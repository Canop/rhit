mod addr;
mod paths;
mod referers;
pub mod summary;
mod status;
mod methods;
mod printer;
mod skin;

pub use {
    printer::*,
};


use {
    crate::*,
};

pub fn print_analysis(
    log_base: &LogBase,
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    if log_base.is_empty() {
        return;
    }
    let log_lines = &log_base.lines;
    let mut popular_paths = false;
    let mut trendy_paths = false;
    for table in printer.tables.clone().into_iter() {
        match table {
            Table::Dates => {
                let histogram = Histogram::from(&log_base);
                histogram.print(printer);
            }
            Table::Status => {
                status::print_status_codes(log_lines, printer, trend_computer);
            }
            Table::RemoteAddresses => {
                addr::print_remote_addresses(log_lines, printer, trend_computer);
            }
            Table::Referers => {
                referers::print_referers(log_lines, printer, trend_computer);
            }
            Table::Paths => {
                popular_paths = true;
            }
            Table::Trends => {
                trendy_paths = true;
            }
            Table::Methods => {
                methods::print_methods(log_lines, printer, trend_computer);
            }
        }
    }
    if popular_paths || trendy_paths {
        if let Some(trend_computer) = trend_computer {
            paths::print_paths(
                &log_base,
                printer,
                trend_computer,
                popular_paths,
                trendy_paths,
            );
        } else {
            paths::print_paths_no_trends(
                &log_base,
                printer,
            );
        }
    }
}
