#[macro_use] extern crate cli_log;

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    rhit::run()?;
    info!("bye");
    Ok(())
}
