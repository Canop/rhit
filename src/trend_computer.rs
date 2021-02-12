use {
    crate::*,
};

const MAX_HISTO_LEN: usize = 20;

pub struct TrendComputer {
    histo_offset: usize, // number of old days skipped
    histo_len: usize, // = ref_len + tail_len
    pub ref_len: usize,
    pub tail_len: usize,
    normalization_factor: f32,
    pub key: Key,
}
impl TrendComputer {
    pub fn new(
        base: &LogBase,
        key: Key,
    ) -> Option<Self> {
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
            normalization_factor: 1f32, // temporary value
            key,
        };
        let counts_per_day = computer.compute_histo_line(&base.lines);
        let (ref_count, tail_count) = computer.compute_ref_tail_counts(&counts_per_day);
        computer.normalization_factor = (ref_count as f32) / (tail_count as f32);
        Some(computer)
    }
    pub fn compute_histo_line<DI: DateIndexed>(&self, lines: &[DI]) -> Vec<u64> {
        let mut counts = vec![0; self.histo_len];
        match self.key {
            Key::Hits => {
                for line in lines {
                    if line.date_idx() < self.histo_offset {
                        continue;
                    }
                    counts[line.date_idx() - self.histo_offset] += 1;
                }
            }
            Key::Bytes => {
                for line in lines {
                    if line.date_idx() < self.histo_offset {
                        continue;
                    }
                    counts[line.date_idx() - self.histo_offset] += line.bytes();
                }
            }
        }
        counts
    }
    fn compute_ref_tail_counts(&self, counts_per_day: &[u64]) -> (u64, u64) {
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
    pub fn compute_trend(&self, lines: &[&LogLine]) -> Trend {
        let sum_per_day = self.compute_histo_line(lines);
        let (ref_count, tail_count) = self.compute_ref_tail_counts(&sum_per_day);
        let value = if ref_count+tail_count == 0 {
            0i32
        } else {
            let tc = (tail_count as f32) * self.normalization_factor;
            let rc = ref_count as f32;
            (1000f32 * (tc - rc) / (rc + tc)) as i32
        };
        Trend { sum_per_day, value, ref_count, tail_count }
    }
}
