// Variation: Memory Range Operations
// From: source/verismo/src/addr_e/range_interface.rs
// Demonstrates: Range start/end/size operations with validation

use vstd::prelude::*;

verus! {

pub const MAX_MEM_SIZE: usize = 0x1000_0000; // 256 MB

pub struct MemRange {
    pub start: usize,
    pub size: usize,
}

impl MemRange {
    pub fn new(start: usize, size: usize) -> (result: Self)
        requires
            start <= MAX_MEM_SIZE,
            size <= MAX_MEM_SIZE,
        ensures
            result.start == start,
            result.size == size,
    {
        MemRange { start, size }
    }

    pub fn get_start(&self) -> (result: usize)
        ensures
            result == self.spec_get_start(),
            result <= MAX_MEM_SIZE,
    {
        if self.start < MAX_MEM_SIZE {
            self.start
        } else {
            MAX_MEM_SIZE
        }
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.spec_get_size(),
            result <= MAX_MEM_SIZE,
    {
        if self.start >= MAX_MEM_SIZE {
            0
        } else if self.size < MAX_MEM_SIZE - self.start {
            self.size
        } else {
            MAX_MEM_SIZE - self.start
        }
    }

    pub open spec fn spec_get_start(&self) -> usize {
        if self.start < MAX_MEM_SIZE { self.start } else { MAX_MEM_SIZE }
    }

    pub open spec fn spec_get_size(&self) -> usize {
        if self.start >= MAX_MEM_SIZE {
            0
        } else if self.size < MAX_MEM_SIZE - self.start {
            self.size
        } else {
            (MAX_MEM_SIZE - self.start) as usize
        }
    }

    pub fn get_end(&self) -> (result: usize)
        ensures
            result == self.spec_get_start() + self.spec_get_size(),
            result <= MAX_MEM_SIZE,
    {
        self.get_start() + self.get_size()
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self.spec_get_size() == 0,
    {
        self.get_size() == 0
    }

    pub fn contains(&self, addr: usize) -> (result: bool)
        ensures
            result <==> (self.spec_get_start() <= addr && addr < self.spec_get_start() + self.spec_get_size()),
    {
        self.get_start() <= addr && addr < self.get_end()
    }
}

fn test_mem_range() {
    let range1 = MemRange::new(0x1000, 0x2000);

    let start = range1.get_start();
    assert(start == 0x1000);

    let size = range1.get_size();
    assert(size == 0x2000);

    let end = range1.get_end();
    assert(end == 0x3000);

    let empty = range1.is_empty();
    assert(!empty);

    let contains1 = range1.contains(0x2000);
    assert(contains1);

    let contains2 = range1.contains(0x5000);
    assert(!contains2);

    let range2 = MemRange::new(0, 0);
    let empty2 = range2.is_empty();
    assert(empty2);

    let range3 = MemRange::new(0x1000, MAX_MEM_SIZE);
    let end3 = range3.get_end();
    assert(end3 <= MAX_MEM_SIZE);
}

} // verus!

fn main() {
    test_mem_range();
}
