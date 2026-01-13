// Variation: Pointer Validity Checking
// From: source/verismo/src/ptr/ptr_u.rs (not_null, is_null, wf_at)
// Demonstrates: Checking if pointers are valid and well-formed

use vstd::prelude::*;

verus! {

pub const INVALID_ADDR: usize = 0;
pub const MIN_VALID_ADDR: usize = 0x1000;
pub const MAX_VALID_ADDR: usize = 0xFFFF_0000;

pub struct ValidatedPtr {
    pub addr: usize,
    pub size: usize,
}

impl ValidatedPtr {
    pub fn new(addr: usize, size: usize) -> (result: Self)
        ensures
            result.addr == addr,
            result.size == size,
    {
        ValidatedPtr { addr, size }
    }

    pub open spec fn id(&self) -> int {
        self.addr as int
    }

    pub open spec fn spec_valid_addr(&self) -> bool {
        self.addr >= MIN_VALID_ADDR && self.addr <= MAX_VALID_ADDR
    }

    pub fn is_valid_addr(&self) -> (result: bool)
        ensures
            result <==> self.spec_valid_addr(),
    {
        self.addr >= MIN_VALID_ADDR && self.addr <= MAX_VALID_ADDR
    }

    pub open spec fn spec_valid_addr_with_size(&self) -> bool {
        self.size > 0 && self.addr >= MIN_VALID_ADDR && self.addr + self.size <= MAX_VALID_ADDR
    }

    pub fn is_valid_addr_with_size(&self) -> (result: bool)
        requires
            self.addr <= usize::MAX - self.size,
        ensures
            result <==> self.spec_valid_addr_with_size(),
    {
        self.size > 0 && self.addr >= MIN_VALID_ADDR && self.addr + self.size <= MAX_VALID_ADDR
    }

    pub open spec fn not_null(&self) -> bool {
        self.spec_valid_addr_with_size()
    }

    pub fn check_not_null(&self) -> (result: bool)
        requires
            self.addr <= usize::MAX - self.size,
        ensures
            result <==> self.not_null(),
    {
        self.is_valid_addr_with_size()
    }

    pub open spec fn is_null(&self) -> bool {
        !self.spec_valid_addr_with_size()
    }

    pub fn check_is_null(&self) -> (result: bool)
        requires
            self.addr <= usize::MAX - self.size,
        ensures
            result <==> self.is_null(),
    {
        !self.is_valid_addr_with_size()
    }

    pub open spec fn wf_at(&self, expected_addr: int) -> bool {
        self.id() == expected_addr && self.not_null()
    }

    pub fn check_wf_at(&self, expected_addr: usize) -> (result: bool)
        requires
            self.addr <= usize::MAX - self.size,
        ensures
            result <==> self.wf_at(expected_addr as int),
    {
        self.addr == expected_addr && self.check_not_null()
    }

    pub open spec fn in_range(&self, min_addr: int, max_addr: int) -> bool {
        min_addr <= self.id() && self.id() + self.size as int <= max_addr
    }

    pub fn check_in_range(&self, min_addr: usize, max_addr: usize) -> (result: bool)
        requires
            min_addr <= max_addr,
            self.addr <= usize::MAX - self.size,
        ensures
            result <==> self.in_range(min_addr as int, max_addr as int),
    {
        min_addr <= self.addr && self.addr + self.size <= max_addr
    }
}

pub struct NullablePtr {
    pub addr: usize,
}

impl NullablePtr {
    pub const fn null() -> (result: Self)
        ensures
            result.addr == INVALID_ADDR,
    {
        NullablePtr { addr: INVALID_ADDR }
    }

    pub fn from_addr(addr: usize) -> (result: Self)
        ensures
            result.addr == addr,
    {
        NullablePtr { addr }
    }

    pub fn is_null(&self) -> (result: bool)
        ensures
            result <==> self.addr == INVALID_ADDR,
    {
        self.addr == INVALID_ADDR
    }

    pub fn is_some(&self) -> (result: bool)
        ensures
            result <==> self.addr != INVALID_ADDR,
    {
        self.addr != INVALID_ADDR
    }

    pub fn get_or_default(&self, default: usize) -> (result: usize)
        ensures
            result == if self.addr == INVALID_ADDR { default } else { self.addr },
    {
        if self.is_null() {
            default
        } else {
            self.addr
        }
    }
}

fn test_pointer_validity() {
    let valid_ptr = ValidatedPtr::new(0x10000, 64);
    let is_valid = valid_ptr.is_valid_addr();
    let not_null = valid_ptr.check_not_null();
    let wf_at = valid_ptr.check_wf_at(0x10000);

    let invalid_ptr = ValidatedPtr::new(0x100, 64);
    let is_invalid = invalid_ptr.check_is_null();

    let in_range = valid_ptr.check_in_range(0x8000, 0x20000);

    let null_ptr = NullablePtr::null();
    let is_null = null_ptr.is_null();

    let some_ptr = NullablePtr::from_addr(0x1000);
    let is_some = some_ptr.is_some();

    let addr = null_ptr.get_or_default(0x2000);
}

} // verus!

fn main() {
    test_pointer_validity();
}
