use {
    super::*,
    crate::*,
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::*,
};

pub struct Printer {
    pub skin: MadSkin,
    pub tables: Tables,
    pub terminal_width: usize,
    pub detail_level: usize,
}

impl Printer {
    pub fn new(args: &args::Args) -> Self {
        let detail_level = args.length;
        let tables = args.tables.clone();
        let terminal_width = terminal_size().0 as usize;
        let color = args.color.value().unwrap_or(!is_output_piped());
        let skin = skin::make_skin(color);
        Self { skin, tables, terminal_width, detail_level }
    }
    pub fn print(
        &self,
        expander: OwningTemplateExpander,
        template: &str,
    ) {
        let template = TextTemplate::from(template);
        let text = expander.expand(&template);
        let fmt_text = FmtText::from_text(&self.skin, text, Some(self.terminal_width));
        print!("{}", fmt_text);
    }
}

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
                addr::print_popular_remote_addresses(log_lines, printer);
            }
            Table::Referers => {
                referers::print_popular_referers(log_lines, printer);
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

fn is_output_piped() -> bool {
    unsafe {
        libc::isatty(libc::STDOUT_FILENO) == 0
    }
}
