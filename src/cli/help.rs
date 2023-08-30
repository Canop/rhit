use {
    crate::{
        args::*,
    },
    clap::CommandFactory,
    termimad::{
        ansi,
        CompoundStyle,
        crossterm::style::{Attribute, Color},
    },
};

static INTRO_TEMPLATE: &str = "
**Rhit** analyzes your nginx logs.

Complete documentation at *https://dystroy.org/rhit*

";

pub fn print() {
    let mut printer = clap_help::Printer::new(Args::command())
        .with("introduction", INTRO_TEMPLATE)
        .without("author");
    let skin = printer.skin_mut();
    skin.headers[0].compound_style.set_fg(ansi(204));
    skin.italic = CompoundStyle::with_attr(Attribute::Underlined);
    skin.bold.set_fg(Color::AnsiValue(204));
    printer.print_help();
}
