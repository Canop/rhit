use {
    crate::{
        Date,
        LogLine,
    },
};

pub trait LineConsumer {
    fn start_eating(
        &mut self,
        _first_date: Date,
    ) {
    }
    fn eat_line(
        &mut self,
        log_line: LogLine,
        raw_line: &str,
        filtered_out: bool,
    );
    fn end_eating(
        &mut self,
    ) {
    }
}

