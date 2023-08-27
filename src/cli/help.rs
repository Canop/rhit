use {
    crate::{
        args::*,
    },
    clap::CommandFactory,
    termimad::ansi,
};

static INTRO_TEMPLATE: &str = "
**Rhit** analyses your nginx logs.

Complete documentation at https://dystroy.org/rhit

";

pub fn print() {
    let mut printer = clap_help::Printer::new(Args::command())
        .with("introduction", INTRO_TEMPLATE)
        .without("author");
    let skin = printer.skin_mut();
    skin.headers[0].compound_style.set_fg(ansi(204));
    skin.bold.set_fg(ansi(204));
    printer.print_help();
}
