mod addr;
mod paths;
mod referers;
pub mod summary;
mod status;
mod methods;

use {
    crate::*,
    crossterm::style::{Attribute::*, Color::*},
    minimad::{Compound, OwningTemplateExpander, TextTemplate},
    termimad::*,
};

pub struct Printer {
    pub skin: MadSkin,
    pub tables: Tables,
    pub terminal_width: usize,
    pub detail_level: usize,
}

impl Printer {
    pub fn new(detail_level: usize, tables: Tables) -> Self {
        let terminal_width = terminal_size().0 as usize;
        let skin = make_skin();
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
                methods::print_methods(log_lines, printer);
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

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[1].compound_style.remove_attr(Underlined);
    skin.italic.remove_attr(Italic);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(AnsiValue(204)); // Magenta is softer
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin.special_chars.insert(
        Compound::raw_str("U").code(),
        StyledChar::from_fg_char(Green, '➚'),
    );
    skin.special_chars.insert(
        Compound::raw_str("D").code(),
        StyledChar::from_fg_char(Red, '➘'),
    );
    skin
}
