use {
    super::*,
    crate::*,
};

pub fn print_methods(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
){
    printer.print_groups(
        "methods",
        "method",
        log_lines,
        &|_| true,
        &|line| line.method,
        trend_computer,
        100, // there should not be more than 100 methods
        false,
    );
}

