use {
    super::*,
    crate::*,
};

pub fn print_remote_addresses(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let n = match printer.detail_level {
        0 => 3,
        1 => 5,
        l => l * 10,
    };
    printer.print_groups(
        "remote addresses",
        "remote address",
        log_lines,
        |_| true,
        |line| line.remote_addr,
        trend_computer,
        n,
        true,
    );
}

