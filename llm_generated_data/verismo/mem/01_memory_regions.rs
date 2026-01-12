// Variation: Memory Region Tracking
// From: source/verismo/src/mem/rawmem_p.rs (range concepts)
// Demonstrates: Basic memory region with start address and size

use vstd::prelude::*;

verus! {

pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
}

impl MemoryRegion {
    pub fn new(start: usize, size: usize) -> (result: Self)
        ensures
            result.start == start,
            result.size == size,
    {
        MemoryRegion { start, size }
    }

    pub fn get_start(&self) -> (result: usize)
        ensures
            result == self.start,
    {
        self.start
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub fn get_end(&self) -> (result: usize)
        requires
            self.start <= usize::MAX - self.size,
        ensures
            result == self.start + self.size,
    {
        self.start + self.size
    }

    pub fn is_valid(&self) -> (result: bool)
        ensures
            result <==> (self.size > 0 && self.start <= usize::MAX - self.size),
    {
        self.size > 0 && self.start <= usize::MAX - self.size
    }

    pub fn contains_addr(&self, addr: usize) -> (result: bool)
        requires
            self.start <= usize::MAX - self.size,
        ensures
            result <==> (self.start <= addr && addr < self.start + self.size),
    {
        self.start <= addr && addr < self.get_end()
    }

    pub fn offset_of(&self, addr: usize) -> (result: Option<usize>)
        requires
            self.start <= usize::MAX - self.size,
        ensures
            match result {
                Some(offset) => self.start + offset == addr && offset < self.size,
                None => !(self.start <= addr && addr < self.start + self.size),
            },
    {
        if self.contains_addr(addr) {
            Some(addr - self.start)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self.size == 0,
    {
        self.size == 0
    }

    pub fn is_single_byte(&self) -> (result: bool)
        ensures
            result <==> self.size == 1,
    {
        self.size == 1
    }
}

fn test_memory_regions() {
    let region1 = MemoryRegion::new(0x1000, 0x100);
    let region2 = MemoryRegion::new(0x2000, 0);

    let start1 = region1.get_start();
    let size1 = region1.get_size();
    let end1 = region1.get_end();

    let valid1 = region1.is_valid();
    let valid2 = region2.is_valid();

    let contains1 = region1.contains_addr(0x1050);
    let contains2 = region1.contains_addr(0x2000);

    let offset1 = region1.offset_of(0x1050);
    let offset2 = region1.offset_of(0x2000);

    let empty1 = region1.is_empty();
    let empty2 = region2.is_empty();

    let single1 = region1.is_single_byte();
    let single2 = MemoryRegion::new(0x1000, 1).is_single_byte();
}

} // verus!

fn main() {
    test_memory_regions();
}
