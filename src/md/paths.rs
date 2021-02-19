use {
    super::*,
    crate::*,
};

pub fn print_paths(
    log_lines: &[LogLine],
    printer: &Printer,
    trend_computer: Option<&TrendComputer>,
) {
    let limit = match printer.detail_level {
        0 => 10,
        l => l * 50,
    };
    let section = Section {
        groups_name: "paths (excluding resources like images, css, etc.)",
        group_key: "path",
        view: View::Limited(limit),
        changes: true,
    };
    printer.print_groups(
        &section,
        log_lines,
        |line| !line.is_resource(),
        |line| &line.path,
        trend_computer,
    );
}
