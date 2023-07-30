
/// A trait for structs which hold the index of a date
pub trait DateIndexed {
    fn date_idx(&self) -> usize;
    fn bytes(&self) -> u64;
}
