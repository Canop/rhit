#[macro_use] extern crate log;

fn main() -> anyhow::Result<()> {
    cli_log::init("rhit");
    rhit::run()?;
    info!("bye");
    Ok(())
}
