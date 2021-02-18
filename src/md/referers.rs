use {
    super::*,
    crate::*,
};

pub fn print_referers(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let limit = match printer.detail_level {
        0 => 5,
        1 => 10,
        l => l * 20,
    };
    let section = Section {
        groups_name: "referrers",
        group_key: "referrer",
        view: View::Limited(limit),
        changes: true,
    };
    printer.print_groups(
        &section,
        log_lines,
        |line| line.referer.len() > 1,
        |line| &line.referer,
        trend_computer,
    );
}
