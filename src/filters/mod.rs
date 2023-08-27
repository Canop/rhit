
mod date_time_filter;
mod status_filter;
mod str_filter;
mod method_filter;
mod time_filter;

pub use {
    date_time_filter::*,
    method_filter::*,
    status_filter::*,
    str_filter::*,
    time_filter::*,
};

use {
    crate::*,
    anyhow::*,
    smallvec::*,
    std::{
        str::FromStr,
    },
};

pub enum Filter {
    //Date(DateFilter),
    DateTime(DateTimeFilter),
    Ip(StrFilter),
    Method(MethodFilter),
    Path(StrFilter),
    Referer(StrFilter),
    Status(StatusFilter),
    Time(TimeFilter),
}

impl Filter {
    pub fn accepts(&self, line: &LogLine) -> bool {
        match self {
            //Self::Date(f) => f.contains(line.date()),
            Self::DateTime(f) => f.contains(line.date_time),
            Self::Ip(f) => f.accepts(&line.remote_addr),
            Self::Method(f) => f.contains(line.method),
            Self::Path(f) => f.accepts(&line.path),
            Self::Referer(f) => f.accepts(&line.referer),
            Self::Status(f) => f.accepts(line.status),
            Self::Time(f) => f.contains(line.time()),
        }
    }
    pub fn field_name(&self) -> &'static str {
        match self {
            Self::DateTime(_) => "date", // it's date_time but clearer as date ?
            Self::Ip(_) => "remote address",
            Self::Method(_) => "method",
            Self::Path(_) => "path",
            Self::Referer(_) => "referer", // it looks like it's the usual orthograph
            Self::Status(_) => "status",
            Self::Time(_) => "time",
        }
    }
}

pub struct Filtering {
    pub pattern: String,
    pub filter: Filter,
    pub removed_count: usize,
}

impl Filtering {
    pub fn new(pattern: &str, filter: Filter) -> Self {
        Self {
            pattern: pattern.to_owned(),
            filter,
            removed_count: 0,
        }
    }
}

pub struct Filterer {
    pub first_date: Date,
    pub filterings: SmallVec<[Filtering; 5]>,
}

impl Filterer {
    pub fn new(
        args: &args::Args,
        first_date: Date,
        last_date: Date,
    ) -> Result<Self> {
        let (default_year, default_month) = unique_year_month(first_date, last_date);
        let mut filterings = SmallVec::new();
        if let Some(s) = &args.date {
            filterings.push(Filtering::new(
                s,
                Filter::DateTime(DateTimeFilter::new(s, default_year, default_month)?),
            ));
        }
        if let Some(s) = &args.ip {
            filterings.push(Filtering::new(
                s,
                Filter::Ip(StrFilter::new(s)?),
            ));
        }
        if let Some(s) = &args.method {
            filterings.push(Filtering::new(
                s,
                Filter::Method(MethodFilter::from_string(s)),
            ));
        }
        if let Some(s) = &args.path {
            filterings.push(Filtering::new(
                s,
                Filter::Path(StrFilter::new(s)?),
            ));
        }
        if let Some(s) = &args.referer {
            filterings.push(Filtering::new(
                s,
                Filter::Referer(StrFilter::new(s)?),
            ));
        }
        if let Some(s) = &args.status {
            filterings.push(Filtering::new(
                s,
                Filter::Status(StatusFilter::from_str(s)?),
            ));
        }
        if let Some(s) = &args.time {
            filterings.push(Filtering::new(
                s,
                Filter::Time(TimeFilter::from_str(s)?),
            ));
        }
        Ok(Self { first_date, filterings })
    }
    /// Return the filter used in rendering to select the day to display
    pub fn date_filter(&self) -> Option<&DateTimeFilter> {
        for i in 0..self.filterings.len() {
            if let Filter::DateTime(f) = &self.filterings[i].filter {
                return Some(f);
            }
        }
        None
    }
    pub fn accepts(&mut self, line: &LogLine) -> bool {
        for filtering in &mut self.filterings {
            if !filtering.filter.accepts(line) {
                filtering.removed_count += 1;
                return false;
            }
        }
        true
    }
    pub fn has_filters(&self) -> bool {
        !self.filterings.is_empty()
    }
}
