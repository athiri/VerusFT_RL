// Variation: Range Comparison and Overlap
// From: source/verismo/src/addr_e/range_interface.rs
// Demonstrates: Range disjoint checking, overlap detection, containment

use vstd::prelude::*;

verus! {

pub struct Range {
    pub start: usize,
    pub size: usize,
}

impl Range {
    pub fn new(start: usize, size: usize) -> (result: Self)
        requires
            start + size <= usize::MAX,
        ensures
            result.start == start,
            result.size == size,
    {
        Range { start, size }
    }

    pub open spec fn spec_end(&self) -> usize {
        (self.start + self.size) as usize
    }

    pub fn end(&self) -> (result: usize)
        requires
            self.start + self.size <= usize::MAX,
        ensures
            result == self.spec_end(),
    {
        self.start + self.size
    }

    pub fn check_disjoint(&self, other: &Range) -> (result: bool)
        requires
            self.start + self.size <= usize::MAX,
            other.start + other.size <= usize::MAX,
        ensures
            result <==> (self.spec_end() <= other.start || other.spec_end() <= self.start || self.size == 0 || other.size == 0),
    {
        self.end() <= other.start || other.end() <= self.start || self.size == 0 || other.size == 0
    }

    pub fn check_overlap(&self, other: &Range) -> (result: bool)
        requires
            self.start + self.size <= usize::MAX,
            other.start + other.size <= usize::MAX,
        ensures
            result <==> !(self.spec_end() <= other.start || other.spec_end() <= self.start || self.size == 0 || other.size == 0),
    {
        !self.check_disjoint(other)
    }

    pub fn check_inside(&self, other: &Range) -> (result: bool)
        requires
            self.start + self.size <= usize::MAX,
            other.start + other.size <= usize::MAX,
        ensures
            result <==> (other.start <= self.start && self.start < other.spec_end() && self.spec_end() <= other.spec_end()),
    {
        other.start <= self.start && self.start < other.end() && self.end() <= other.end()
    }

    pub fn less_than(&self, other: &Range) -> (result: bool)
        ensures
            result <==> (self.start < other.start || (self.start == other.start && self.size < other.size)),
    {
        if self.start < other.start {
            true
        } else if self.start == other.start {
            self.size < other.size
        } else {
            false
        }
    }
}

fn test_range_comparison() {
    let range1 = Range::new(0x1000, 0x1000);
    let range2 = Range::new(0x2000, 0x1000);
    let range3 = Range::new(0x1500, 0x800);
    let range4 = Range::new(0x1000, 0x3000);

    let disjoint = range1.check_disjoint(&range2);
    assert(disjoint);

    let overlap1 = range1.check_overlap(&range3);
    assert(overlap1);

    let overlap2 = range1.check_overlap(&range2);
    assert(!overlap2);

    let inside1 = range3.check_inside(&range1);
    assert(inside1);

    let inside2 = range1.check_inside(&range4);
    assert(inside2);

    let less1 = range1.less_than(&range2);
    assert(less1);

    let less2 = range2.less_than(&range1);
    assert(!less2);

    let empty_range = Range::new(0x1000, 0);
    let disjoint_empty = empty_range.check_disjoint(&range1);
    assert(disjoint_empty);
}

} // verus!

fn main() {
    test_range_comparison();
}
