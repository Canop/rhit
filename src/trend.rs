use {
    crate::*,
    std::cmp::Ordering,
};

const MAX_HISTO_LEN: usize = 20;

#[derive(Debug, Clone, Eq)]
pub struct Trend {
    pub counts_per_day: Vec<usize>,
    pub value: i32,
    pub ref_count: usize,
    pub tail_count: usize,
}

impl Trend {
    pub fn max_day_count(&self) -> usize {
        *self.counts_per_day.iter().max().unwrap()
    }
    pub fn sum(&self) -> usize {
        self.ref_count + self.tail_count
    }
    pub fn markdown(&self) -> &'static str {
        if self.value > 200 {
            if self.value > 900 {
                "`U` `U` `U`"
            } else if self.value > 500 {
                "`U` `U`"
            } else {
                "`U`"
            }
        } else if self.value < -200 {
            if self.value < -900 {
                "`D` `D` `D`"
            } else if self.value < -500 {
                "`D` `D`"
            } else {
                "`D`"
            }
        } else {
            " "
        }
    }
}

impl Ord for Trend {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else if self.value > 0 {
            self.sum().cmp(&other.sum())
        } else {
            other.sum().cmp(&self.sum())
        }
    }
}

impl PartialOrd for Trend {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Trend {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sum() == other.sum()
    }
}

pub struct TrendComputer {
    histo_offset: usize, // number of old days skipped
    histo_len: usize, // = ref_len + tail_len
    pub ref_len: usize,
    pub tail_len: usize,
    normalization_factor: f32,
}
impl TrendComputer {
    pub fn new(base: &LogBase) -> Option<Self> {
        let dc = base.day_count();
        if dc < 4 {
            return None;
        }
        let histo_len = dc.min(MAX_HISTO_LEN);
        let histo_offset = base.day_count() - histo_len;
        let tail_len = 2;
        let ref_len = histo_len - tail_len;
        let mut computer = Self {
            tail_len,
            ref_len,
            histo_len,
            histo_offset,
            normalization_factor: 1f32, // temporary
        };
        let counts_per_day = computer.compute_histo_line(&base.lines);
        let (ref_count, tail_count) = computer.compute_ref_tail_counts(&counts_per_day);
        computer.normalization_factor = (ref_count as f32) / (tail_count as f32);
        Some(computer)
    }
    pub fn compute_histo_line<DI: DateIndexed>(&self, lines: &Vec<DI>) -> Vec<usize> {
        let mut counts = vec![0; self.histo_len];
        for line in lines {
            if line.date_idx() < self.histo_offset {
                continue;
            }
            counts[line.date_idx() - self.histo_offset] += 1;
        }
        counts
    }
    fn compute_ref_tail_counts(&self, counts_per_day: &[usize]) -> (usize, usize) {
        let (mut ref_count, mut tail_count) = (0, 0);
        let mut idx = 0;
        while idx < self.ref_len {
            ref_count += counts_per_day[idx];
            idx += 1;
        }
        while idx < self.histo_len {
            tail_count += counts_per_day[idx];
            idx += 1;
        }
        (ref_count, tail_count)
    }
    pub fn compute_trend(&self, lines: &Vec<&LogLine>) -> Trend {
        let counts_per_day = self.compute_histo_line(lines);
        let (ref_count, tail_count) = self.compute_ref_tail_counts(&counts_per_day);
        let value = if ref_count+tail_count == 0 {
            0i32
        } else {
            let tc = (tail_count as f32) * self.normalization_factor;
            let rc = ref_count as f32;
            (1000f32 * (tc - rc) / (rc + tc)) as i32
        };
        Trend { counts_per_day, value, ref_count, tail_count }
    }
}
