// Variation: Pointer Conversion Operations
// From: source/verismo/src/ptr/ptr_e.rs (from_usize, to_usize, to, as_u64)
// Demonstrates: Converting between pointer representations

use vstd::prelude::*;

verus! {

pub const INVALID_ADDR: usize = 0;

pub struct TypedPtr<V> {
    pub addr: usize,
    pub phantom: core::marker::PhantomData<V>,
}

impl<V> core::marker::Copy for TypedPtr<V> {}

impl<V> Clone for TypedPtr<V> {
    fn clone(&self) -> Self {
        TypedPtr {
            addr: self.addr,
            phantom: core::marker::PhantomData,
        }
    }
}

impl<V> TypedPtr<V> {
    pub fn from_usize(addr: usize) -> (result: Self)
        ensures
            result.addr == addr,
    {
        TypedPtr {
            addr,
            phantom: core::marker::PhantomData,
        }
    }

    pub const fn nullptr() -> (result: Self)
        ensures
            result.addr == INVALID_ADDR,
    {
        TypedPtr {
            addr: INVALID_ADDR,
            phantom: core::marker::PhantomData,
        }
    }

    pub fn to_usize(&self) -> (result: usize)
        ensures
            result == self.addr,
    {
        self.addr
    }

    pub fn as_u64(&self) -> (result: u64)
        ensures
            result == self.addr as u64,
    {
        self.addr as u64
    }

    pub fn to<V2>(&self) -> (result: TypedPtr<V2>)
        ensures
            result.addr == self.addr,
    {
        TypedPtr::from_usize(self.to_usize())
    }

    pub open spec fn id(&self) -> int {
        self.addr as int
    }

    pub fn is_null(&self) -> (result: bool)
        ensures
            result <==> self.addr == INVALID_ADDR,
    {
        self.addr == INVALID_ADDR
    }

    pub fn equals<V2>(&self, other: &TypedPtr<V2>) -> (result: bool)
        ensures
            result <==> self.addr == other.addr,
    {
        self.addr == other.addr
    }
}

pub fn cast_ptr<V1, V2>(ptr: TypedPtr<V1>) -> (result: TypedPtr<V2>)
    ensures
        result.addr == ptr.addr,
{
    ptr.to::<V2>()
}

pub fn ptr_add<V>(ptr: TypedPtr<V>, offset: usize) -> (result: TypedPtr<V>)
    requires
        ptr.addr <= usize::MAX - offset,
    ensures
        result.addr == ptr.addr + offset,
{
    TypedPtr::from_usize(ptr.to_usize() + offset)
}

pub fn ptr_sub<V>(ptr: TypedPtr<V>, offset: usize) -> (result: TypedPtr<V>)
    requires
        ptr.addr >= offset,
    ensures
        result.addr == ptr.addr - offset,
{
    TypedPtr::from_usize(ptr.to_usize() - offset)
}

pub fn ptr_distance<V>(start: TypedPtr<V>, end: TypedPtr<V>) -> (result: Option<usize>)
    ensures
        match result {
            Some(dist) => {
                &&& start.addr <= end.addr
                &&& dist == end.addr - start.addr
            },
            None => start.addr > end.addr,
        },
{
    if start.addr <= end.addr {
        Some(end.addr - start.addr)
    } else {
        None
    }
}

pub struct PtrRange<V> {
    pub start: TypedPtr<V>,
    pub end: TypedPtr<V>,
}

impl<V> PtrRange<V> {
    pub fn new(start: TypedPtr<V>, end: TypedPtr<V>) -> (result: Self)
        requires
            start.addr <= end.addr,
        ensures
            result.start.addr == start.addr,
            result.end.addr == end.addr,
    {
        PtrRange { start, end }
    }

    pub fn size(&self) -> (result: usize)
        requires
            self.start.addr <= self.end.addr,
        ensures
            result == self.end.addr - self.start.addr,
    {
        self.end.addr - self.start.addr
    }

    pub fn contains<V2>(&self, ptr: TypedPtr<V2>) -> (result: bool)
        requires
            self.start.addr <= self.end.addr,
        ensures
            result <==> (self.start.addr <= ptr.addr && ptr.addr < self.end.addr),
    {
        self.start.addr <= ptr.addr && ptr.addr < self.end.addr
    }

    pub fn is_empty(&self) -> (result: bool)
        requires
            self.start.addr <= self.end.addr,
        ensures
            result <==> self.start.addr == self.end.addr,
    {
        self.start.addr == self.end.addr
    }
}

fn test_pointer_conversion() {
    let ptr1: TypedPtr<u64> = TypedPtr::from_usize(0x1000);
    let addr = ptr1.to_usize();
    let u64_val = ptr1.as_u64();

    let ptr2: TypedPtr<u32> = ptr1.to::<u32>();
    let equals = ptr1.equals(&ptr2);

    let null_ptr: TypedPtr<u8> = TypedPtr::nullptr();
    let is_null = null_ptr.is_null();

    let ptr3 = cast_ptr::<u64, u32>(ptr1);

    let ptr4 = ptr_add(ptr1, 0x100);
    let ptr5 = ptr_sub(ptr4, 0x50);

    let distance = ptr_distance(ptr1, ptr4);

    let range = PtrRange::new(ptr1, ptr4);
    let size = range.size();
    let contains = range.contains(ptr5);
    let is_empty = range.is_empty();
}

} // verus!

fn main() {
    test_pointer_conversion();
}
