mod addr;
mod paths;
mod referers;
pub mod summary;
mod status;

use {
    crate::*,
    crossterm::style::{Attribute::*, Color::*},
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::*,
};

pub fn print(expander: OwningTemplateExpander, template: &str, skin: &MadSkin) {
    let (width, _) = terminal_size();
    let template = TextTemplate::from(template);
    let text = expander.expand(&template);
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
}

pub fn print_analysis(
    log_base: &LogBase,
    skin: &MadSkin,
    tables: &Tables,
    detail_level: usize,
) {
    let log_lines = &log_base.lines;
    for table in tables.clone().into_iter() {
        match table {
            Table::Dates => {
                let histogram = Histogram::of_days(&log_base);
                histogram.print(&skin);
            }
            Table::Status => {
                status::print_status_codes(log_lines, skin);
            }
            Table::RemoteAddresses => {
                addr::print_popular_remote_addresses(log_lines, detail_level, skin);
            }
            Table::Referers => {
                referers::print_popular_referers(log_lines, detail_level, skin);
            }
            Table::Paths => {
                paths::print_popular_paths(log_lines, detail_level, skin);
            }
        }
    }
}

pub fn make_skin() -> MadSkin {
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
