// Variation: Size Validation and Tracking
// From: source/verismo/src/allocator/buddy.rs, locked.rs
// Demonstrates: Allocation size validation, minimum size requirements

use vstd::prelude::*;

verus! {

pub const MIN_ALLOC_SIZE: usize = 16;
pub const MAX_ALLOC_SIZE: usize = 1usize << 30; // 1GB

pub open spec fn spec_is_valid_size(size: usize) -> bool {
    MIN_ALLOC_SIZE <= size && size <= MAX_ALLOC_SIZE
}

pub fn is_valid_size(size: usize) -> (result: bool)
    ensures
        result <==> spec_is_valid_size(size),
{
    MIN_ALLOC_SIZE <= size && size <= MAX_ALLOC_SIZE
}

pub fn normalize_size(size: usize) -> (result: usize)
    ensures
        result >= MIN_ALLOC_SIZE,
        result >= size,
{
    if size < MIN_ALLOC_SIZE {
        MIN_ALLOC_SIZE
    } else {
        size
    }
}

pub fn can_allocate(requested_size: usize, available_size: usize) -> (result: bool)
    ensures
        result <==> (spec_is_valid_size(requested_size) && requested_size <= available_size),
{
    is_valid_size(requested_size) && requested_size <= available_size
}

pub fn fits_in_block(size: usize, block_size: usize) -> (result: bool)
    requires
        block_size >= MIN_ALLOC_SIZE,
    ensures
        result <==> size <= block_size,
{
    size <= block_size
}

pub fn calculate_usable_size(block_size: usize, header_size: usize) -> (result: usize)
    requires
        block_size >= header_size,
    ensures
        result == block_size - header_size,
        result <= block_size,
{
    block_size - header_size
}

pub fn is_within_range(value: usize, min: usize, max: usize) -> (result: bool)
    ensures
        result <==> (min <= value && value <= max),
{
    min <= value && value <= max
}

pub fn is_too_large(size: usize) -> (result: bool)
    ensures
        result <==> size > MAX_ALLOC_SIZE,
{
    size > MAX_ALLOC_SIZE
}

pub fn is_too_small(size: usize) -> (result: bool)
    ensures
        result <==> size < MIN_ALLOC_SIZE,
{
    size < MIN_ALLOC_SIZE
}

fn test_size_validation() {
    let valid1_check = is_valid_size(1024);
    let valid2_check = is_valid_size(MIN_ALLOC_SIZE);

    let invalid1_check = is_valid_size(8);

    let norm1 = normalize_size(10);
    let norm2 = normalize_size(1024);

    let can_alloc1 = can_allocate(1024, 2048);
    let can_alloc2 = can_allocate(8, 2048);

    let fits1 = fits_in_block(100, 1024);
    let fits2 = fits_in_block(2048, 1024);

    let usable = calculate_usable_size(1024, 64);

    let in_range1 = is_within_range(100, 10, 1000);
    let in_range2 = is_within_range(1, 10, 1000);

    let too_large1 = is_too_large(MAX_ALLOC_SIZE);
    let too_large2 = is_too_large(MIN_ALLOC_SIZE);

    let too_small1 = is_too_small(8);
    let too_small2 = is_too_small(1024);
}

} // verus!

fn main() {
    test_size_validation();
}
