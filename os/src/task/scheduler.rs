use core::cmp::Ordering;

#[derive(Eq, Debug)]
pub struct Stride {
    pub stride: usize,
    pub pid: usize,
}

impl Ord for Stride {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.stride == other.stride {
            other.pid.cmp(&self.pid)
        } else {
            other.stride.cmp(&self.stride)
        }
    }
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Stride {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
