use {
    crate::*,
    std::{
        path::PathBuf,
    },
};


struct RawPrinter {
}

impl LineConsumer for RawPrinter {
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

pub fn print_raw_lines(
    path: &[PathBuf],
    args: &args::Args,
) -> Result<(), RhitError> {
    let mut printer = RawPrinter{};
    let mut file_reader = FileReader::new(path, args, &mut printer)?;
    time!("reading files", file_reader.read_all_files())?;
    Ok(())
}
