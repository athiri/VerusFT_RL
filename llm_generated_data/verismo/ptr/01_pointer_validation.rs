// Variation: Pointer Validation
// From: source/verismo/src/ptr/ptr_e.rs
// Demonstrates: Pointer validity checking, null pointer handling

use vstd::prelude::*;

verus! {

pub const NULL_PTR: usize = 0;
pub const MAX_VALID_PTR: usize = usize::MAX / 2; // Simplified validity limit

pub fn is_null(ptr: usize) -> (result: bool)
    ensures
        result <==> ptr == NULL_PTR,
{
    ptr == NULL_PTR
}

pub fn is_not_null(ptr: usize) -> (result: bool)
    ensures
        result <==> ptr != NULL_PTR,
{
    ptr != NULL_PTR
}

pub fn is_canonical(ptr: usize) -> (result: bool)
    ensures
        result <==> ptr <= MAX_VALID_PTR,
{
    ptr <= MAX_VALID_PTR
}

pub fn ptr_is_valid(ptr: usize, size: usize) -> (result: bool)
    requires
        size > 0,
        size <= MAX_VALID_PTR,
    ensures
        result <==> (ptr != NULL_PTR && ptr + size - 1 <= MAX_VALID_PTR),
{
    ptr != NULL_PTR && ptr <= MAX_VALID_PTR - size + 1
}

pub fn ptr_add_offset(ptr: usize, offset: usize) -> (result: Option<usize>)
    ensures
        match result {
            Some(new_ptr) => new_ptr == ptr + offset && new_ptr <= MAX_VALID_PTR,
            None => offset > MAX_VALID_PTR || ptr > MAX_VALID_PTR - offset,
        },
{
    if offset <= MAX_VALID_PTR && ptr <= MAX_VALID_PTR - offset {
        Some(ptr + offset)
    } else {
        None
    }
}

pub fn ptr_sub_offset(ptr: usize, offset: usize) -> (result: Option<usize>)
    ensures
        match result {
            Some(new_ptr) => new_ptr == ptr - offset,
            None => ptr < offset,
        },
{
    if ptr >= offset {
        Some(ptr - offset)
    } else {
        None
    }
}

pub fn ptr_distance(ptr1: usize, ptr2: usize) -> (result: usize)
    requires
        ptr1 >= ptr2,
    ensures
        result == ptr1 - ptr2,
{
    ptr1 - ptr2
}

fn test_pointers() {
    let null = NULL_PTR;
    let check_null = is_null(null);
    assert(check_null);

    let valid_ptr = 0x1000usize;
    let check_not_null = is_not_null(valid_ptr);
    assert(check_not_null);

    let check_canonical = is_canonical(valid_ptr);
    assert(check_canonical);

    let check_valid = ptr_is_valid(valid_ptr, 64);
    assert(check_valid);

    let new_ptr = ptr_add_offset(valid_ptr, 0x100);
    assert(new_ptr.is_some());
    assert(new_ptr.unwrap() == 0x1100usize);

    let sub_ptr = ptr_sub_offset(valid_ptr, 0x100);
    assert(sub_ptr.is_some());
    assert(sub_ptr.unwrap() == 0xF00usize);

    let dist = ptr_distance(0x2000usize, 0x1000usize);
    assert(dist == 0x1000usize);
}

} // verus!

fn main() {
    test_pointers();
}
