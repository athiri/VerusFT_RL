// Variation: Lock Range Management
// From: source/verismo/src/lock/spin_perm_s.rs (ptr_range concept)
// Demonstrates: Managing lock ranges and address calculations

use vstd::prelude::*;

verus! {

pub struct LockRange {
    pub base: usize,
    pub size: usize,
}

impl LockRange {
    pub fn new(base: usize, size: usize) -> (result: Self)
        ensures
            result.base == base,
            result.size == size,
    {
        LockRange { base, size }
    }

    pub fn get_base(&self) -> (result: usize)
        ensures
            result == self.base,
    {
        self.base
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub fn get_end(&self) -> (result: usize)
        requires
            self.base <= usize::MAX - self.size,
        ensures
            result == self.base + self.size,
    {
        self.base + self.size
    }

    pub fn contains(&self, addr: usize) -> (result: bool)
        requires
            self.base <= usize::MAX - self.size,
        ensures
            result <==> (self.base <= addr && addr < self.base + self.size),
    {
        self.base <= addr && addr < self.get_end()
    }

    pub fn overlaps(&self, other: &LockRange) -> (result: bool)
        requires
            self.base <= usize::MAX - self.size,
            other.base <= usize::MAX - other.size,
        ensures
            result <==> !(self.base + self.size <= other.base || other.base + other.size <= self.base),
    {
        !(self.get_end() <= other.base || other.get_end() <= self.base)
    }

    pub fn is_aligned(&self, alignment: usize) -> (result: bool)
        requires
            alignment > 0,
        ensures
            result <==> self.base % alignment == 0,
    {
        self.base % alignment == 0
    }

    pub fn get_offset(&self, addr: usize) -> (result: Option<usize>)
        requires
            self.base <= usize::MAX - self.size,
        ensures
            match result {
                Some(offset) => self.base + offset == addr && offset < self.size,
                None => !(self.base <= addr && addr < self.base + self.size),
            },
    {
        if self.contains(addr) {
            Some(addr - self.base)
        } else {
            None
        }
    }

    pub fn is_within(&self, outer: &LockRange) -> (result: bool)
        requires
            self.base <= usize::MAX - self.size,
            outer.base <= usize::MAX - outer.size,
        ensures
            result <==> (outer.base <= self.base && self.base + self.size <= outer.base + outer.size),
    {
        outer.base <= self.base && self.get_end() <= outer.get_end()
    }
}

fn test_lock_range() {
    let range1 = LockRange::new(0x1000, 0x100);
    let range2 = LockRange::new(0x2000, 0x100);
    let range3 = LockRange::new(0x1050, 0x50);

    let base1 = range1.get_base();
    let size1 = range1.get_size();
    let end1 = range1.get_end();

    let contains1 = range1.contains(0x1050);
    let contains2 = range1.contains(0x2000);

    let overlaps_12 = range1.overlaps(&range2);
    let overlaps_13 = range1.overlaps(&range3);

    let aligned_16 = range1.is_aligned(16);
    let aligned_256 = range1.is_aligned(256);

    let offset1 = range1.get_offset(0x1050);
    let offset2 = range1.get_offset(0x2000);

    let within_13 = range3.is_within(&range1);
    let within_12 = range1.is_within(&range2);
}

} // verus!

fn main() {
    test_lock_range();
}
