// Variation: Address Range Operations
// From: source/verismo/src/addr_e/range_interface.rs
// Demonstrates: Range splitting, merging, and adjustment

use vstd::prelude::*;

verus! {

pub struct AddrRange {
    pub start: usize,
    pub size: usize,
}

impl AddrRange {
    pub fn new(start: usize, size: usize) -> (result: Self)
        requires
            start + size <= usize::MAX,
        ensures
            result.start == start,
            result.size == size,
    {
        AddrRange { start, size }
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

    pub fn split_at(&self, offset: usize) -> (result: Option<(AddrRange, AddrRange)>)
        requires
            self.start + self.size <= usize::MAX,
        ensures
            match result {
                Some((first, second)) => {
                    &&& offset > 0 && offset < self.size
                    &&& first.start == self.start
                    &&& first.size == offset
                    &&& second.start == self.start + offset
                    &&& second.size == self.size - offset
                },
                None => offset == 0 || offset >= self.size,
            },
    {
        if offset > 0 && offset < self.size {
            let first = AddrRange::new(self.start, offset);
            let second = AddrRange::new(self.start + offset, self.size - offset);
            Some((first, second))
        } else {
            None
        }
    }

    pub fn can_merge(&self, other: &AddrRange) -> (result: bool)
        requires
            self.start + self.size <= usize::MAX,
            other.start + other.size <= usize::MAX,
        ensures
            result <==> (self.spec_end() == other.start || other.spec_end() == self.start),
    {
        self.end() == other.start || other.end() == self.start
    }

    pub fn merge(&self, other: &AddrRange) -> (result: Option<AddrRange>)
        requires
            self.start + self.size <= usize::MAX,
            other.start + other.size <= usize::MAX,
        ensures
            match result {
                Some(merged) => {
                    &&& (self.spec_end() == other.start || other.spec_end() == self.start)
                    &&& merged.start == if self.start < other.start { self.start } else { other.start }
                    &&& merged.size == self.size + other.size
                },
                None => !(self.spec_end() == other.start || other.spec_end() == self.start),
            },
    {
        if self.can_merge(other) {
            let new_start = if self.start < other.start { self.start } else { other.start };
            Some(AddrRange::new(new_start, self.size + other.size))
        } else {
            None
        }
    }

    pub fn resize(&self, new_size: usize) -> (result: AddrRange)
        requires
            self.start + new_size <= usize::MAX,
        ensures
            result.start == self.start,
            result.size == new_size,
    {
        AddrRange::new(self.start, new_size)
    }

    pub fn shift_right(&self, offset: usize) -> (result: Option<AddrRange>)
        requires
            self.start + self.size <= usize::MAX,
        ensures
            match result {
                Some(shifted) => {
                    &&& shifted.start == self.start + offset
                    &&& shifted.size == self.size
                    &&& shifted.start + shifted.size <= usize::MAX
                },
                None => self.start + offset + self.size > usize::MAX,
            },
    {
        if offset <= usize::MAX - self.start && self.start + offset <= usize::MAX - self.size {
            Some(AddrRange::new(self.start + offset, self.size))
        } else {
            None
        }
    }

    pub fn shift_left(&self, offset: usize) -> (result: Option<AddrRange>)
        requires
            self.start + self.size <= usize::MAX,
        ensures
            match result {
                Some(shifted) => {
                    &&& shifted.start == self.start - offset
                    &&& shifted.size == self.size
                },
                None => self.start < offset,
            },
    {
        if self.start >= offset {
            Some(AddrRange::new(self.start - offset, self.size))
        } else {
            None
        }
    }
}

fn test_range_operations() {
    let range1 = AddrRange::new(0x1000, 0x2000);

    let split = range1.split_at(0x1000);
    assert(split.is_some());
    let (first, second) = split.unwrap();
    assert(first.start == 0x1000);
    assert(first.size == 0x1000);
    assert(second.start == 0x2000);
    assert(second.size == 0x1000);

    let no_split = range1.split_at(0);
    assert(no_split.is_none());

    let range2 = AddrRange::new(0x3000, 0x1000);
    let can_merge = range1.can_merge(&range2);
    assert(can_merge);

    let merged = range1.merge(&range2);
    assert(merged.is_some());
    let m = merged.unwrap();
    assert(m.start == 0x1000);
    assert(m.size == 0x3000);

    let range3 = AddrRange::new(0x5000, 0x1000);
    let cannot_merge = range1.can_merge(&range3);
    assert(!cannot_merge);

    let resized = range1.resize(0x3000);
    assert(resized.start == 0x1000);
    assert(resized.size == 0x3000);

    let shifted_right = range1.shift_right(0x1000);
    assert(shifted_right.is_some());
    let sr = shifted_right.unwrap();
    assert(sr.start == 0x2000);

    let shifted_left = range1.shift_left(0x500);
    assert(shifted_left.is_some());
    let sl = shifted_left.unwrap();
    assert(sl.start == 0xB00);
}

} // verus!

fn main() {
    test_range_operations();
}
