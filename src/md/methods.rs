use {
    super::*,
    crate::*,
};

pub fn print_methods(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let section = Section {
        groups_name: "methods",
        group_key: "method",
        view: View::Full,
        changes: false,
    };
    printer.print_groups(
        &section,
        log_lines,
        |_| true,
        |line| line.method,
        trend_computer,
    );
}

