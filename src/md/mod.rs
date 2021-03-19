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
    base: &LogBase,
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    if base.is_empty() {
        return;
    }
    // note about times: the first markdown template expansion (whatever it is)
    // costs a lot when there's a filtering. I don't know exactly why.
    let lines = &base.lines;
    for field in &printer.fields.0 {
        match field {
            Field::Dates => {
                let histogram = Histogram::from(&base);
                time!(
                    "histogram printing",
                    histogram.print(printer),
                );
            }
            Field::Methods => {
                time!(
                    "print_methods",
                    methods::print_methods(lines, printer, trend_computer),
                );
            }
            Field::Status => {
                time!(
                    "print_status_codes",
                    status::print_status_codes(lines, printer, trend_computer),
                );
            }
            Field::Ip => {
                time!(
                    "print_remote_addresses",
                    addr::print_remote_addresses(lines, printer, trend_computer),
                );
            }
            Field::Referers => {
                time!(
                    "print_referers",
                    referers::print_referers(lines, printer, trend_computer),
                );
            }
            Field::Paths => {
                time!(
                    "print_paths",
                    paths::print_paths(lines, printer, trend_computer),
                );
            }
        }
    }
}
