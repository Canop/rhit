mod addr;
mod paths;
mod referers;
pub mod summary;
mod status;
mod methods;
mod printer;
mod section;
mod skin;

pub use {
    printer::*,
    section::*,
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
    if printer.fields.contains(Field::Dates) {
        Histogram::from(&log_base).print(printer);
    }
    if printer.fields.contains(Field::Methods) {
        methods::print_methods(log_lines, printer, trend_computer);
    }
    if printer.fields.contains(Field::Status) {
        status::print_status_codes(log_lines, printer, trend_computer);
    }
    if printer.fields.contains(Field::RemoteAddresses) {
        addr::print_remote_addresses(log_lines, printer, trend_computer);
    }
    if printer.fields.contains(Field::Referers) {
        referers::print_referers(log_lines, printer, trend_computer);
    }
    if printer.fields.contains(Field::Paths) {
        paths::print_paths(log_lines, printer, trend_computer);
    }
}
