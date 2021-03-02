use core::cmp::Ordering;

#[derive(Eq, Ord, Debug)]
pub struct Stride {
    pub stride: usize,
    pub pid: usize,
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.stride.partial_cmp(&self.stride)
    }
}

impl PartialEq for Stride {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}