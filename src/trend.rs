use {
    std::cmp::Ordering,
};

#[derive(Debug, Clone, Eq)]
pub struct Trend {
    pub sum_per_day: Vec<u64>,
    pub value: i32,
    pub ref_count: u64,
    pub tail_count: u64,
}

impl Trend {
    pub fn max_day_count(&self) -> u64 {
        *self.sum_per_day.iter().max().unwrap()
    }
    pub fn sum(&self) -> u64 {
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

