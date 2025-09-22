#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Span<T: Copy> {
    pub kind: T,
    pub start: usize,
    pub end: usize,
}

use std::ops::Range;

impl<T: Default + Copy> Default for Span<T> {
    fn default() -> Self {
        Self {
            kind: T::default(),
            start: 0,
            end: 0,
        }
    }
}

impl<T: Copy> Span<T> {
    pub(crate) fn new(kind: T, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    #[inline(always)]
    pub(crate) fn kind(&self) -> T {
        self.kind
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}
