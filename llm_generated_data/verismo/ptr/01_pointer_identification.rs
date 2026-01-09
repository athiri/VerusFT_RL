// Variation: Pointer Identification
// From: source/verismo/src/ptr/def_s.rs (SnpPPtr id, range_id)
// Demonstrates: Pointer address identification and range calculation

use vstd::prelude::*;

verus! {

pub struct SimplePtr {
    pub addr: usize,
}

impl SimplePtr {
    pub fn new(addr: usize) -> (result: Self)
        ensures
            result.addr == addr,
    {
        SimplePtr { addr }
    }

    pub open spec fn id(&self) -> int {
        self.addr as int
    }

    pub fn get_id(&self) -> (result: usize)
        ensures
            result as int == self.id(),
    {
        self.addr
    }

    pub open spec fn range_id(&self, size: nat) -> (int, nat) {
        (self.id(), size)
    }

    pub fn is_aligned(&self, alignment: usize) -> (result: bool)
        requires
            alignment > 0,
        ensures
            result <==> self.addr % alignment == 0,
    {
        self.addr % alignment == 0
    }

    pub fn offset(&self, off: usize) -> (result: Self)
        requires
            self.addr <= usize::MAX - off,
        ensures
            result.id() == self.id() + off as int,
    {
        SimplePtr { addr: self.addr + off }
    }

    pub fn distance_to(&self, other: &Self) -> (result: usize)
        requires
            self.addr <= other.addr,
        ensures
            result as int == other.id() - self.id(),
    {
        other.addr - self.addr
    }
}

pub struct PtrWithSize {
    pub ptr: SimplePtr,
    pub size: usize,
}

impl PtrWithSize {
    pub fn new(ptr: SimplePtr, size: usize) -> (result: Self)
        ensures
            result.ptr.id() == ptr.id(),
            result.size == size,
    {
        PtrWithSize { ptr, size }
    }

    pub open spec fn range(&self) -> (int, nat) {
        (self.ptr.id(), self.size as nat)
    }

    pub fn get_range(&self) -> (result: (usize, usize))
        ensures
            result.0 as int == self.ptr.id(),
            result.1 == self.size,
    {
        (self.ptr.addr, self.size)
    }

    pub fn contains(&self, addr: usize) -> (result: bool)
        requires
            self.ptr.addr <= usize::MAX - self.size,
        ensures
            result <==> (self.ptr.addr <= addr && addr < self.ptr.addr + self.size),
    {
        self.ptr.addr <= addr && addr < self.ptr.addr + self.size
    }

    pub fn overlaps(&self, other: &Self) -> (result: bool)
        requires
            self.ptr.addr <= usize::MAX - self.size,
            other.ptr.addr <= usize::MAX - other.size,
    {
        let self_end = self.ptr.addr + self.size;
        let other_end = other.ptr.addr + other.size;
        !(self_end <= other.ptr.addr || other_end <= self.ptr.addr)
    }
}

fn test_pointer_identification() {
    let ptr1 = SimplePtr::new(0x1000);
    let id1 = ptr1.get_id();

    let aligned = ptr1.is_aligned(16);

    let ptr2 = ptr1.offset(0x100);
    let distance = ptr1.distance_to(&ptr2);

    let ptr_with_size = PtrWithSize::new(ptr1, 64);
    let range = ptr_with_size.get_range();

    let contains = ptr_with_size.contains(0x1010);

    let ptr3 = PtrWithSize::new(SimplePtr::new(0x1040), 32);
    let overlaps = ptr_with_size.overlaps(&ptr3);
}

} // verus!

fn main() {
    test_pointer_identification();
}
