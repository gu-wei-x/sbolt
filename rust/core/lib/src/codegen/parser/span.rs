use crate::types::Location;
use std::ops::Range;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Span<T: Copy> {
    kind: T,
    start: usize,
    end: usize,
    location: Location,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 0, column: 0 }
    }
}

impl Location {
    pub(crate) fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl<T: Copy> Span<T> {
    pub(crate) fn new(kind: T, start: usize, end: usize, location: Location) -> Self {
        Self {
            kind,
            start,
            end,
            location,
        }
    }

    #[inline(always)]
    pub(crate) fn kind(&self) -> T {
        self.kind
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub(crate) fn location(&self) -> Location {
        self.location
    }
}
