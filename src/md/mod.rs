mod addr;
mod paths;
mod referers;
pub mod summary;
mod status;
mod methods;

use {
    crate::*,
    crossterm::style::{Attribute::*, Color::*},
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
) {
    let log_lines = &log_base.lines;
    for table in printer.tables.clone().into_iter() {
        match table {
            Table::Dates => {
                let histogram = Histogram::of_days(&log_base);
                histogram.print(printer);
            }
            Table::Status => {
                status::print_status_codes(log_lines, printer);
            }
            Table::RemoteAddresses => {
                addr::print_popular_remote_addresses(log_lines, printer);
            }
            Table::Referers => {
                referers::print_popular_referers(log_lines, printer);
            }
            Table::Paths => {
                paths::print_popular_paths(log_lines, printer);
            }
            Table::Methods => {
                methods::print_methods(log_lines, printer);
            }
        }
    }
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[1].compound_style.remove_attr(Underlined);
    skin.italic.remove_attr(Italic);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}
