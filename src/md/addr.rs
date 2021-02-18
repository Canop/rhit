use {
    super::*,
    crate::*,
};

pub fn print_remote_addresses(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let limit = match printer.detail_level {
        0 => 3,
        1 => 5,
        l => l * 10,
    };
    let section = Section {
        groups_name: "remote IP addresses",
        group_key: "IP address",
        view: View::Limited(limit),
        changes: true,
    };
    printer.print_groups(
        &section,
        log_lines,
        |_| true,
        |line| line.remote_addr,
        trend_computer,
    );
}

