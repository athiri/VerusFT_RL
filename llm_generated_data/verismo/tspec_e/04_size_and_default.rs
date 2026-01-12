// Variation: Size and Default Traits
// From: source/verismo/src/tspec_e/size_e.rs, default.rs, type_test.rs
// Demonstrates: SpecSize trait and Default specifications

use vstd::prelude::*;

verus! {

pub trait SpecDefault: Sized {
    spec fn spec_default() -> Self;
}

impl SpecDefault for u8 {
    open spec fn spec_default() -> Self {
        0u8
    }
}

impl SpecDefault for u16 {
    open spec fn spec_default() -> Self {
        0u16
    }
}

impl SpecDefault for u32 {
    open spec fn spec_default() -> Self {
        0u32
    }
}

impl SpecDefault for u64 {
    open spec fn spec_default() -> Self {
        0u64
    }
}

impl SpecDefault for usize {
    open spec fn spec_default() -> Self {
        0usize
    }
}

impl SpecDefault for bool {
    open spec fn spec_default() -> Self {
        false
    }
}

impl<T: SpecDefault, U: SpecDefault> SpecDefault for (T, U) {
    open spec fn spec_default() -> Self {
        (T::spec_default(), U::spec_default())
    }
}

pub struct TypeInfo<T> {
    pub phantom: Ghost<T>,
}

impl<T> TypeInfo<T> {
    pub fn new() -> (result: Self)
    {
        TypeInfo {
            phantom: Ghost(arbitrary()),
        }
    }
}

impl<T: SpecDefault> TypeInfo<T> {
    pub open spec fn default_value() -> T {
        T::spec_default()
    }
}

pub struct DefaultValue<T> {
    pub value: T,
}

impl<T: SpecDefault> DefaultValue<T> {
    pub open spec fn is_default(&self) -> bool {
        self.value == T::spec_default()
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

impl<T: Copy> DefaultValue<T> {
    pub fn new(value: T) -> (result: Self)
        ensures
            result.value == value,
    {
        DefaultValue { value }
    }

    pub fn get_value(&self) -> (result: T)
        ensures
            result == self.value,
    {
        self.value
    }
}

pub struct FieldSizes {
    pub field1_size: usize,
    pub field2_size: usize,
    pub total_size: usize,
}

impl FieldSizes {
    pub fn new(field1_size: usize, field2_size: usize, total_size: usize) -> (result: Self)
        ensures
            result.field1_size == field1_size,
            result.field2_size == field2_size,
            result.total_size == total_size,
    {
        FieldSizes {
            field1_size,
            field2_size,
            total_size,
        }
    }

    pub open spec fn sum_matches(&self) -> bool {
        self.field1_size + self.field2_size <= self.total_size
    }

    pub open spec fn wf(&self) -> bool {
        self.sum_matches()
    }

    pub fn get_field1_size(&self) -> (result: usize)
        ensures
            result == self.field1_size,
    {
        self.field1_size
    }

    pub fn get_field2_size(&self) -> (result: usize)
        ensures
            result == self.field2_size,
    {
        self.field2_size
    }

    pub fn get_total_size(&self) -> (result: usize)
        ensures
            result == self.total_size,
    {
        self.total_size
    }
}

pub struct Alignment {
    pub size: usize,
    pub align: usize,
}

impl Alignment {
    pub fn new(size: usize, align: usize) -> (result: Self)
        ensures
            result.size == size,
            result.align == align,
    {
        Alignment { size, align }
    }

    pub fn for_u8() -> (result: Self)
        ensures
            result.size == 1,
            result.align == 1,
    {
        Alignment { size: 1, align: 1 }
    }

    pub fn for_u16() -> (result: Self)
        ensures
            result.size == 2,
            result.align == 2,
    {
        Alignment { size: 2, align: 2 }
    }

    pub fn for_u32() -> (result: Self)
        ensures
            result.size == 4,
            result.align == 4,
    {
        Alignment { size: 4, align: 4 }
    }

    pub fn for_u64() -> (result: Self)
        ensures
            result.size == 8,
            result.align == 8,
    {
        Alignment { size: 8, align: 8 }
    }

    pub open spec fn is_aligned(&self, addr: usize) -> bool
        recommends
            self.align > 0,
    {
        addr % self.align == 0
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.size > 0
        &&& self.align > 0
        &&& self.size >= self.align
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub fn get_align(&self) -> (result: usize)
        ensures
            result == self.align,
    {
        self.align
    }
}

pub struct TypeLayout {
    pub size: usize,
    pub align: usize,
    pub field_count: usize,
}

impl TypeLayout {
    pub fn new(size: usize, align: usize, field_count: usize) -> (result: Self)
        ensures
            result.size == size,
            result.align == align,
            result.field_count == field_count,
    {
        TypeLayout {
            size,
            align,
            field_count,
        }
    }

    pub fn primitive(size: usize) -> (result: Self)
        ensures
            result.size == size,
            result.align == size,
            result.field_count == 0,
    {
        TypeLayout {
            size,
            align: size,
            field_count: 0,
        }
    }

    pub open spec fn is_primitive(&self) -> bool {
        self.field_count == 0 && self.size == self.align
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.size > 0
        &&& self.align > 0
        &&& self.size >= self.align
    }

    pub fn get_size(&self) -> (result: usize)
        ensures
            result == self.size,
    {
        self.size
    }

    pub fn get_align(&self) -> (result: usize)
        ensures
            result == self.align,
    {
        self.align
    }

    pub fn get_field_count(&self) -> (result: usize)
        ensures
            result == self.field_count,
    {
        self.field_count
    }
}

pub struct PaddingInfo {
    pub required_size: usize,
    pub aligned_size: usize,
}

impl PaddingInfo {
    pub fn new(required_size: usize, aligned_size: usize) -> (result: Self)
        ensures
            result.required_size == required_size,
            result.aligned_size == aligned_size,
    {
        PaddingInfo {
            required_size,
            aligned_size,
        }
    }

    pub open spec fn padding(&self) -> int {
        self.aligned_size as int - self.required_size as int
    }

    pub open spec fn has_padding(&self) -> bool {
        self.required_size < self.aligned_size
    }

    pub open spec fn wf(&self) -> bool {
        self.required_size <= self.aligned_size
    }

    pub fn get_required_size(&self) -> (result: usize)
        ensures
            result == self.required_size,
    {
        self.required_size
    }

    pub fn get_aligned_size(&self) -> (result: usize)
        ensures
            result == self.aligned_size,
    {
        self.aligned_size
    }
}

fn test_size_and_default() {
    let type_info: TypeInfo<u64> = TypeInfo::new();

    let default_u64: DefaultValue<u64> = DefaultValue::new(0);
    let val = default_u64.get_value();

    let field_sizes = FieldSizes::new(8, 8, 16);
    let f1_size = field_sizes.get_field1_size();
    let f2_size = field_sizes.get_field2_size();
    let total = field_sizes.get_total_size();

    let align = Alignment::for_u64();
    let size = align.get_size();
    let align_val = align.get_align();

    let layout = TypeLayout::primitive(8);
    let layout_size = layout.get_size();
    let layout_align = layout.get_align();
    let field_count = layout.get_field_count();

    let padding = PaddingInfo::new(11, 16);
    let req_size = padding.get_required_size();
    let aligned_size = padding.get_aligned_size();

    proof {
        assert(TypeInfo::<u64>::default_value() == 0);
        assert(TypeInfo::<bool>::default_value() == false);
        assert(TypeInfo::<(u32, u64)>::default_value() == (0u32, 0u64));
        assert(val == 0);
        assert(default_u64.is_default());
        assert(default_u64.wf());
        assert(f1_size == 8);
        assert(f2_size == 8);
        assert(total == 16);
        assert(field_sizes.sum_matches());
        assert(field_sizes.wf());
        assert(size == 8);
        assert(align_val == 8);
        assert(align.is_aligned(0));
        assert(align.is_aligned(16));
        assert(!align.is_aligned(4));
        assert(align.wf());
        assert(layout_size == 8);
        assert(layout_align == 8);
        assert(field_count == 0);
        assert(layout.is_primitive());
        assert(layout.wf());
        assert(req_size == 11);
        assert(aligned_size == 16);
        assert(padding.padding() == 5);
        assert(padding.has_padding());
        assert(padding.wf());
    }
}

} // verus!

fn main() {
    test_size_and_default();
}
