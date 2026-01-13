// Variation: Free List Tracking
// From: source/verismo/src/allocator/buddy.rs
// Demonstrates: Simple free block tracking for allocator

use vstd::prelude::*;

verus! {

pub struct FreeBlock {
    pub addr: usize,
    pub size: usize,
}

impl FreeBlock {
    pub fn new(addr: usize, size: usize) -> (result: Self)
        ensures
            result.addr == addr,
            result.size == size,
    {
        FreeBlock { addr, size }
    }

    pub fn end(&self) -> (result: usize)
        requires
            self.addr + self.size <= usize::MAX,
        ensures
            result == self.addr + self.size,
    {
        self.addr + self.size
    }

    pub fn contains(&self, addr: usize) -> (result: bool)
        requires
            self.addr + self.size <= usize::MAX,
        ensures
            result <==> (self.addr <= addr && addr < self.addr + self.size),
    {
        self.addr <= addr && addr < self.end()
    }

    pub fn can_satisfy(& self, requested_size: usize) -> (result: bool)
        ensures
            result <==> self.size >= requested_size,
    {
        self.size >= requested_size
    }
}

pub struct SimpleFreeList {
    pub blocks: Vec<FreeBlock>,
}

impl SimpleFreeList {
    pub fn new() -> (result: Self)
        ensures
            result.blocks@.len() == 0,
    {
        SimpleFreeList { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, addr: usize, size: usize)
        requires
            size > 0,
        ensures
            self.blocks@.len() == old(self).blocks@.len() + 1,
    {
        self.blocks.push(FreeBlock::new(addr, size));
    }

    pub fn find_block(&self, min_size: usize) -> (result: Option<usize>)
        ensures
            match result {
                Some(idx) => {
                    &&& idx < self.blocks@.len()
                    &&& self.blocks@[idx as int].size >= min_size
                },
                None => {
                    forall|i: int| 0 <= i < self.blocks@.len() ==> self.blocks@[i].size < min_size
                },
            },
    {
        let mut i = 0;
        while i < self.blocks.len()
            invariant
                0 <= i <= self.blocks@.len(),
                forall|j: int| 0 <= j < i ==> self.blocks@[j].size < min_size,
            decreases self.blocks@.len() - i
        {
            if self.blocks[i].can_satisfy(min_size) {
                return Some(i);
            }
            i = i + 1;
        }
        None
    }

    pub fn remove_block(&mut self, idx: usize) -> (result: FreeBlock)
        requires
            idx < old(self).blocks@.len(),
        ensures
            result == old(self).blocks@[idx as int],
            self.blocks@.len() == old(self).blocks@.len() - 1,
    {
        self.blocks.remove(idx)
    }

    pub fn total_free(&self) -> (result: usize)
        ensures
            result <= usize::MAX,
    {
        let mut total = 0;
        let mut i = 0;
        while i < self.blocks.len()
            invariant
                0 <= i <= self.blocks@.len(),
            decreases self.blocks@.len() - i
        {
            if total <= usize::MAX - self.blocks[i].size {
                total = total + self.blocks[i].size;
            }
            i = i + 1;
        }
        total
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures
            result <==> self.blocks@.len() == 0,
    {
        self.blocks.len() == 0
    }

    pub fn count(&self) -> (result: usize)
        ensures
            result == self.blocks@.len(),
    {
        self.blocks.len()
    }
}

fn test_free_list() {
    let mut free_list = SimpleFreeList::new();
    let empty = free_list.is_empty();

    free_list.add_block(0x1000, 0x1000);
    free_list.add_block(0x3000, 0x2000);
    free_list.add_block(0x6000, 0x800);

    let count = free_list.count();

    let found = free_list.find_block(0x1500);
    let not_found = free_list.find_block(0x5000);

    let total = free_list.total_free();

    let block = FreeBlock::new(0x1000, 0x1000);
    let contains = block.contains(0x1500);
    let not_contains = block.contains(0x3000);
    let can_satisfy = block.can_satisfy(0x800);
}

} // verus!

fn main() {
    test_free_list();
}
