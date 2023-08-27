use {
    crate::*,
    anyhow::*,
    std::{
        path::PathBuf,
    },
};

/// A printer writing lines as a JSON array
#[derive(Default)]
struct JsonPrinter {
    written: usize,
}

impl LineConsumer for JsonPrinter {
    fn eat_line(
        &mut self,
        line: LogLine,
        _raw_line: &str,
        filtered_out: bool,
    ) {
        if filtered_out { return; }
        if self.written > 0 {
            print!(", ");
        } else {
            println!("\n[");
        }
        print!(r#"  {{
    "date": "{}",
    "time": "{}",
    "remote_addr": "{}",
    "method": "{}",
    "path": "{}",
    "status": "{}",
    "bytes_sent": {},
    "referer": "{}"
  }}"#,
            line.date(),
            line.time(),
            line.remote_addr,
            line.method,
            line.path,
            line.status,
            line.bytes_sent,
            line.referer,
        );
        self.written += 1;
    }
    fn end_eating(
        &mut self,
    ) {
        println!("\n]");
    }
}

pub fn print_json_lines(
    path: &[PathBuf],
    args: &args::Args,
) -> Result<()> {
    let mut printer = JsonPrinter::default();
    let mut file_reader = FileReader::new(path, args, &mut printer)?;
    time!("reading files", file_reader.read_all_files())?;
    Ok(())
}

