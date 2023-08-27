use {
    crate::*,
    anyhow::*,
    std::{
        path::PathBuf,
    },
};

/// A printer writing lines as a CSV file
struct CsvPrinter {}

impl LineConsumer for CsvPrinter {
    fn start_eating(
        &mut self,
        _first_date: Date,
    ) {
        println!("date,time,remote address,method,path,status,bytes sent,referer");
    }
    fn eat_line(
        &mut self,
        line: LogLine,
        _raw_line: &str,
        filtered_out: bool,
    ) {
        if filtered_out { return; }
        println!(
            r#"{},{},{},{},"{}",{},{},"{}""#,
            line.date(),
            line.time(),
            line.remote_addr,
            line.method,
            line.path,
            line.status,
            line.bytes_sent,
            line.referer,
        );
    }
}

pub fn print_csv_lines(
    path: &[PathBuf],
    args: &args::Args,
) -> Result<()> {
    let mut printer = CsvPrinter{};
    let mut file_reader = FileReader::new(path, args, &mut printer)?;
    time!("reading files", file_reader.read_all_files())?;
    Ok(())
}

