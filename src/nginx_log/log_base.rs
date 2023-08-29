use {
    crate::*,
    std::{
        path::PathBuf,
    },
};

/// the content for a base being built
#[derive(Default)]
struct BaseContent {
    lines: Vec<LogLine>,
    bar_idx: usize,
    unfiltered_histogram: DateHistogram,
    filtered_histogram: DateHistogram,
}

impl LineConsumer for BaseContent {
    fn start_eating(
        &mut self,
        first_date: Date,
    ) {
        self.unfiltered_histogram.bars.push(DateBar::new(first_date));
        self.filtered_histogram.bars.push(DateBar::new(first_date));
    }
    fn eat_line(
        &mut self,
        mut log_line: LogLine,
        _raw_line: &str,
        filtered_out: bool,
    ) {
        let ubars = &mut self.unfiltered_histogram.bars;
        let fbars = &mut self.filtered_histogram.bars;
        // both histograms are synchronized, we create
        // bars even when there's no filtered hit
        if log_line.date() != ubars[self.bar_idx].date {
            ubars.push(DateBar::new(log_line.date()));
            fbars.push(DateBar::new(log_line.date()));
            self.bar_idx += 1;
        }
        ubars[self.bar_idx].hits += 1;
        ubars[self.bar_idx].bytes_sent += log_line.bytes_sent;
        if !filtered_out {
            fbars[self.bar_idx].hits += 1;
            fbars[self.bar_idx].bytes_sent += log_line.bytes_sent;
            log_line.date_idx = self.bar_idx;
            self.lines.push(log_line);
        }
    }
}

pub struct LogBase {
    pub dates: Vec<Date>, // all the days covering the observed period
    pub filterer: Filterer,
    pub lines: Vec<LogLine>,
    pub filtered_histogram: DateHistogram,
    pub filtered_count: u64,
    pub unfiltered_histogram: DateHistogram,
    pub unfiltered_count: u64,
}

impl LogBase {
    pub fn new(
        paths: &[PathBuf],
        args: &args::Args,
    ) -> Result<Self, RhitError> {
        let mut base_content = BaseContent::default();
        let mut file_reader = FileReader::new(paths, args, &mut base_content)?;
        time!("reading files", file_reader.read_all_files())?;
        let filterer = file_reader.filterer();
        let BaseContent {lines, unfiltered_histogram, filtered_histogram, ..} = base_content;
        let mut unfiltered_count = 0;
        let mut dates = Vec::new();
        for bar in &unfiltered_histogram.bars {
            unfiltered_count += bar.hits;
            dates.push(bar.date);
        }
        if unfiltered_count == 0 {
            return Err(RhitError::NoHitInPaths(paths.to_vec()));
        }
        let filtered_count = filtered_histogram.total_hits();
        Ok(Self {
            dates,
            filterer,
            lines,
            filtered_histogram,
            filtered_count,
            unfiltered_histogram,
            unfiltered_count,
        })
    }
    pub fn start_time(&self) -> Date {
        self.dates[0]
    }
    pub fn end_time(&self) -> Date {
        self.dates[self.dates.len() - 1]
    }
    pub fn day_count(&self) -> usize {
        self.dates.len()
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

