use {
    crate::*,
    anyhow::*,
    std::{
        path::Path,
    },
};


struct LinePrinter {
}

impl LineConsumer for LinePrinter {
    fn start_eating(
        &mut self,
        _first_date: Date,
    ) {
        // nothing to do here
    }
    fn eat_line(
        &mut self,
        _log_line: LogLine,
        raw_line: &str,
        filtered_out: bool,
    ) {
        if !filtered_out {
            print!("{}", raw_line);
        }
    }
}

pub fn print_lines(
    path: &Path,
    args: &args::Args,
) -> Result<()> {
    let mut printer = LinePrinter{};
    let mut file_reader = FileReader::new(path, args, &mut printer)?;
    time!("reading files", file_reader.read_all_files())?;
    Ok(())
}
