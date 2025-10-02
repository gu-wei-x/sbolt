#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Span<T: Copy> {
    kind: T,
    start: usize,
    end: usize,
    coordinate: (usize, usize),
}

use std::ops::Range;

impl<T: Default + Copy> Default for Span<T> {
    fn default() -> Self {
        Self {
            kind: T::default(),
            start: 0,
            end: 0,
            coordinate: (0, 0),
        }
    }
}

impl<T: Copy> Span<T> {
    pub(crate) fn new(kind: T, start: usize, end: usize, coordinate: (usize, usize)) -> Self {
        Self {
            kind,
            start,
            end,
            coordinate: coordinate,
        }
    }

    #[inline(always)]
    pub(crate) fn kind(&self) -> T {
        self.kind
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub(crate) fn coordinate(&self) -> (usize, usize) {
        self.coordinate
    }
}
