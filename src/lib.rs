
#[macro_use] extern crate cli_log;
#[macro_use] extern crate lazy_regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate minimad;
#[macro_use] extern crate termimad;

mod cli;
mod date;
mod date_idx;
mod filters;
mod histogram;
mod histo_line;
mod key;
mod leak;
mod line_group;
mod method;
pub mod md;
mod nginx_log;
mod fields;
mod trend;
mod trend_computer;

#[global_allocator]
static ALLOC: leak::LeakingAllocator = leak::LeakingAllocator::new();

pub use {
    cli::*,
    date::*,
    date_idx::*,
    filters::*,
    histogram::*,
    histo_line::*,
    key::*,
    line_group::*,
    method::*,
    nginx_log::*,
    fields::*,
    trend::*,
    trend_computer::*,
};
