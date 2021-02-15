use {
    super::*,
    crate::*,
};

pub fn print_referers(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let n = match printer.detail_level {
        0 => 5,
        1 => 10,
        l => l * 20,
    };
    printer.print_groups(
        "referers",
        "referer",
        log_lines,
        |line| line.referer.len() > 1,
        |line| &line.referer,
        trend_computer,
        n,
        true,
    );
}
