// Variation: SpecSize Trait
// From: source/verismo/src/tspec/size_s.rs
// Demonstrates: SpecSize trait for compile-time size specifications

use vstd::prelude::*;

verus! {

pub trait SpecSize {
    spec fn spec_size_of() -> nat;
}

impl SpecSize for u8 {
    open spec fn spec_size_of() -> nat {
        1
    }
}

impl SpecSize for u16 {
    open spec fn spec_size_of() -> nat {
        2
    }
}

impl SpecSize for u32 {
    open spec fn spec_size_of() -> nat {
        4
    }
}

impl SpecSize for u64 {
    open spec fn spec_size_of() -> nat {
        8
    }
}

impl SpecSize for usize {
    open spec fn spec_size_of() -> nat {
        8
    }
}

impl SpecSize for i8 {
    open spec fn spec_size_of() -> nat {
        1
    }
}

impl SpecSize for i16 {
    open spec fn spec_size_of() -> nat {
        2
    }
}

impl SpecSize for i32 {
    open spec fn spec_size_of() -> nat {
        4
    }
}

impl SpecSize for i64 {
    open spec fn spec_size_of() -> nat {
        8
    }
}

impl SpecSize for bool {
    open spec fn spec_size_of() -> nat {
        1
    }
}

impl<T1: SpecSize, T2: SpecSize> SpecSize for (T1, T2) {
    open spec fn spec_size_of() -> nat {
        T1::spec_size_of() + T2::spec_size_of()
    }
}

impl<T: SpecSize> SpecSize for Option<T> {
    open spec fn spec_size_of() -> nat {
        1 + T::spec_size_of()
    }
}

pub struct SizedBuffer<T> {
    pub data: Vec<u8>,
    pub phantom: Ghost<T>,
}

impl<T: SpecSize> SizedBuffer<T> {
    pub fn new(data: Vec<u8>) -> (result: Self)
        ensures
            result.data@ == data@,
    {
        SizedBuffer {
            data,
            phantom: Ghost(arbitrary()),
        }
    }

    pub fn empty() -> (result: Self)
        ensures
            result.data@.len() == 0,
    {
        SizedBuffer {
            data: Vec::new(),
            phantom: Ghost(arbitrary()),
        }
    }

    pub fn len(&self) -> (result: usize)
        ensures
            result == self.data@.len(),
    {
        self.data.len()
    }

    pub open spec fn capacity(&self) -> nat {
        T::spec_size_of()
    }

    pub open spec fn has_capacity(&self) -> bool {
        self.data@.len() == T::spec_size_of()
    }

    pub open spec fn wf(&self) -> bool {
        self.data@.len() <= T::spec_size_of()
    }
}

pub struct TypedSize<T> {
    pub size: usize,
    pub phantom: Ghost<T>,
}

impl<T: SpecSize> TypedSize<T> {
    pub fn new(size: usize) -> (result: Self)
        ensures
            result.size == size,
    {
        TypedSize {
            size,
            phantom: Ghost(arbitrary()),
        }
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub open spec fn matches_type(&self) -> bool {
        self.size == T::spec_size_of()
    }

    pub open spec fn wf(&self) -> bool {
        self.size > 0 && self.size <= 8192
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.size > 0 && self.size <= 8192
    }
}

pub struct SizeConstraint<T> {
    pub min_size: usize,
    pub max_size: usize,
    pub phantom: Ghost<T>,
}

impl<T: SpecSize> SizeConstraint<T> {
    pub fn new(min_size: usize, max_size: usize) -> (result: Self)
        ensures
            result.min_size == min_size,
            result.max_size == max_size,
    {
        SizeConstraint {
            min_size,
            max_size,
            phantom: Ghost(arbitrary()),
        }
    }

    pub fn get_min_size(&self) -> (result: usize)
        ensures
            result == self.min_size,
    {
        self.min_size
    }

    pub fn get_max_size(&self) -> (result: usize)
        ensures
            result == self.max_size,
    {
        self.max_size
    }

    pub open spec fn allows_size(&self, size: usize) -> bool {
        self.min_size <= size <= self.max_size
    }

    pub fn check_allows_size(&self, size: usize) -> (result: bool)
        ensures
            result <==> self.allows_size(size),
    {
        self.min_size <= size && size <= self.max_size
    }

    pub open spec fn type_fits(&self) -> bool {
        self.min_size <= T::spec_size_of() <= self.max_size
    }

    pub open spec fn wf(&self) -> bool {
        self.min_size <= self.max_size
    }

    pub fn check_wf(&self) -> (result: bool)
        ensures
            result <==> self.wf(),
    {
        self.min_size <= self.max_size
    }
}

pub struct AlignedSize<T> {
    pub size: usize,
    pub alignment: usize,
    pub phantom: Ghost<T>,
}

impl<T: SpecSize> AlignedSize<T> {
    pub fn new(size: usize, alignment: usize) -> (result: Self)
        ensures
            result.size == size,
            result.alignment == alignment,
    {
        AlignedSize {
            size,
            alignment,
            phantom: Ghost(arbitrary()),
        }
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub fn get_alignment(&self) -> (result: usize)
        ensures
            result == self.alignment,
    {
        self.alignment
    }

    pub open spec fn is_aligned(&self) -> bool {
        self.alignment > 0 && self.size % self.alignment == 0
    }

    pub fn check_is_aligned(&self) -> (result: bool)
        requires
            self.alignment > 0,
        ensures
            result <==> self.is_aligned(),
    {
        self.size % self.alignment == 0
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.alignment > 0
        &&& self.size > 0
        &&& self.is_aligned()
    }

    pub fn check_wf(&self) -> (result: bool)
        requires
            self.alignment > 0,
        ensures
            result <==> (self.size > 0 && self.is_aligned()),
    {
        self.size > 0 && self.check_is_aligned()
    }
}

fn test_spec_size() {
    let u8_buf: SizedBuffer<u8> = SizedBuffer::new(Vec::new());
    let u8_len = u8_buf.len();

    let u64_buf: SizedBuffer<u64> = SizedBuffer::empty();

    let typed_size: TypedSize<u32> = TypedSize::new(4);
    let size = typed_size.get_size();
    let typed_wf = typed_size.check_wf();

    let constraint: SizeConstraint<u64> = SizeConstraint::new(8, 8);
    let min = constraint.get_min_size();
    let max = constraint.get_max_size();
    let allows = constraint.check_allows_size(8);
    let constraint_wf = constraint.check_wf();

    let aligned: AlignedSize<u64> = AlignedSize::new(8, 8);
    let aligned_size = aligned.get_size();
    let alignment = aligned.get_alignment();
    let is_aligned = aligned.check_is_aligned();
    let aligned_wf = aligned.check_wf();

    proof {
        assert(u8::spec_size_of() == 1);
        assert(u16::spec_size_of() == 2);
        assert(u32::spec_size_of() == 4);
        assert(u64::spec_size_of() == 8);
        assert(u8_buf.wf());
        assert(u8_len == 0);
        assert(!u64_buf.has_capacity());
        assert(size == 4);
        assert(typed_size.matches_type());
        assert(typed_wf);
        assert(min == 8);
        assert(max == 8);
        assert(allows);
        assert(constraint.type_fits());
        assert(constraint_wf);
        assert(aligned_size == 8);
        assert(alignment == 8);
        assert(is_aligned);
        assert(aligned_wf);
        assert(u8_buf.capacity() == 1);
        assert(<(u32, u64)>::spec_size_of() == 12);
    }
}

} // verus!

fn main() {
    test_spec_size();
}
