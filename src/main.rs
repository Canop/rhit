#[macro_use] extern crate cli_log;

fn main() -> Result<(), rhit::RhitError> {
    init_cli_log!();
    rhit::run()?;
    info!("bye");
    Ok(())
}
