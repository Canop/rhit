
/// A trait for structs which hold the index of a date
/// FIXME this thing is ridiculous, I need somebody knowing rust to fix it
pub trait DateIndexed {
    fn date_idx(&self) -> usize;
    fn bytes(&self) -> u64;
}
