// Variation: Allocation State Tracking
// From: source/verismo/src/allocator concepts
// Demonstrates: Tracking allocated and free memory blocks

use vstd::prelude::*;

verus! {

pub struct AllocationInfo {
    pub addr: usize,
    pub size: usize,
    pub is_allocated: bool,
}

impl AllocationInfo {
    pub fn new(addr: usize, size: usize, is_allocated: bool) -> (result: Self)
        ensures
            result.addr == addr,
            result.size == size,
            result.is_allocated == is_allocated,
    {
        AllocationInfo { addr, size, is_allocated }
    }

    pub fn mark_allocated(&mut self)
        ensures
            self.is_allocated,
            self.addr == old(self).addr,
            self.size == old(self).size,
    {
        self.is_allocated = true;
    }

    pub fn mark_free(&mut self)
        ensures
            !self.is_allocated,
            self.addr == old(self).addr,
            self.size == old(self).size,
    {
        self.is_allocated = false;
    }

    pub fn overlaps(&self, other: &AllocationInfo) -> (result: bool)
        requires
            self.addr + self.size <= usize::MAX,
            other.addr + other.size <= usize::MAX,
        ensures
            result <==> !(self.addr + self.size <= other.addr || other.addr + other.size <= self.addr),
    {
        !(self.addr + self.size <= other.addr || other.addr + other.size <= self.addr)
    }
}

pub struct AllocatorState {
    pub allocations: Vec<AllocationInfo>,
    pub total_memory: usize,
}

impl AllocatorState {
    pub fn new(total_memory: usize) -> (result: Self)
        ensures
            result.allocations@.len() == 0,
            result.total_memory == total_memory,
    {
        AllocatorState {
            allocations: Vec::new(),
            total_memory,
        }
    }

    pub fn record_allocation(&mut self, addr: usize, size: usize)
        requires
            size > 0,
        ensures
            self.allocations@.len() == old(self).allocations@.len() + 1,
            self.total_memory == old(self).total_memory,
    {
        self.allocations.push(AllocationInfo::new(addr, size, true));
    }

    pub fn record_free(&mut self, addr: usize) -> (result: bool)
        ensures
            result ==> self.allocations@.len() == old(self).allocations@.len(),
            self.total_memory == old(self).total_memory,
    {
        let mut i = 0;
        while i < self.allocations.len()
            invariant
                0 <= i <= self.allocations@.len(),
                self.allocations@.len() == old(self).allocations@.len(),
                self.total_memory == old(self).total_memory,
            decreases self.allocations@.len() - i
        {
            if self.allocations[i].addr == addr && self.allocations[i].is_allocated {
                self.allocations.set(i, AllocationInfo::new(addr, self.allocations[i].size, false));
                return true;
            }
            i = i + 1;
        }
        false
    }

    pub fn is_allocated(&self, addr: usize) -> (result: bool)
    {
        let mut i = 0;
        while i < self.allocations.len()
            invariant
                0 <= i <= self.allocations@.len(),
            decreases self.allocations@.len() - i
        {
            if self.allocations[i].addr == addr && self.allocations[i].is_allocated {
                return true;
            }
            i = i + 1;
        }
        false
    }

    pub fn count_allocated(&self) -> (result: usize)
        ensures
            result <= self.allocations@.len(),
    {
        let mut count = 0;
        let mut i = 0;
        while i < self.allocations.len()
            invariant
                0 <= i <= self.allocations@.len(),
                count <= i,
            decreases self.allocations@.len() - i
        {
            if self.allocations[i].is_allocated {
                count = count + 1;
            }
            i = i + 1;
        }
        count
    }

    pub fn allocated_bytes(&self) -> (result: usize)
    {
        let mut total = 0;
        let mut i = 0;
        while i < self.allocations.len()
            invariant
                0 <= i <= self.allocations@.len(),
            decreases self.allocations@.len() - i
        {
            if self.allocations[i].is_allocated && total <= usize::MAX - self.allocations[i].size {
                total = total + self.allocations[i].size;
            }
            i = i + 1;
        }
        total
    }

    pub fn has_capacity(&self, size: usize) -> (result: bool)
    {
        let used = self.allocated_bytes();
        used <= self.total_memory && size <= self.total_memory - used
    }
}

fn test_allocation_state() {
    let mut state = AllocatorState::new(0x10000);

    state.record_allocation(0x1000, 0x100);
    state.record_allocation(0x2000, 0x200);

    let count = state.count_allocated();

    let is_alloc = state.is_allocated(0x1000);

    let freed = state.record_free(0x1000);

    let count_after = state.count_allocated();

    let used = state.allocated_bytes();

    let has_cap = state.has_capacity(0x100);

    let info1 = AllocationInfo::new(0x1000, 0x100, true);
    let info2 = AllocationInfo::new(0x1500, 0x100, false);
    let overlaps = info1.overlaps(&info2);
}

} // verus!

fn main() {
    test_allocation_state();
}
