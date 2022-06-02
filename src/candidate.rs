use crate::topcode::Code;

/// A candidate [TopCode].
pub(crate) struct Candidate {
    pub x: usize,
    pub y: usize,
}

impl Candidate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
