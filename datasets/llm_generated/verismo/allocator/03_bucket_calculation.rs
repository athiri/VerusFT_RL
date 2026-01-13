// Variation: Bucket and Order Calculations
// From: source/verismo/src/allocator/buddy.rs
// Demonstrates: Size-to-bucket mapping for buddy allocator

use vstd::prelude::*;

verus! {

pub const MIN_BLOCK_SIZE: usize = 16;
pub const MAX_ORDER: usize = 32;

pub fn order_to_size(order: usize) -> (result: usize)
    requires
        order <= 31,
    ensures
        result == 1usize << order,
{
    1usize << order
}

pub fn bucket_to_size(bucket: usize) -> (result: usize)
    requires
        bucket >= 4,
        bucket <= 31,
    ensures
        result == 1usize << bucket,
{
    1usize << bucket
}

pub fn is_valid_bucket(bucket: usize) -> (result: bool)
    ensures
        result <==> (bucket >= 4 && bucket < MAX_ORDER),
{
    bucket >= 4 && bucket < MAX_ORDER
}

pub fn size_is_bucket_aligned(size: usize, bucket: usize) -> (result: bool)
    requires
        bucket >= 4,
        bucket <= 31,
{
    let bucket_size = bucket_to_size(bucket);
    size <= bucket_size
}

pub fn get_max_bucket() -> (result: usize)
    ensures
        result == MAX_ORDER - 1,
{
    MAX_ORDER - 1
}

pub fn get_min_bucket() -> (result: usize)
    ensures
        result == 4,
{
    4
}

pub fn get_min_block_size() -> (result: usize)
    ensures
        result == MIN_BLOCK_SIZE,
{
    MIN_BLOCK_SIZE
}

pub fn calculate_bucket_size(bucket: usize) -> (result: Option<usize>)
{
    if bucket >= 4 && bucket <= 31 {
        Some(1usize << bucket)
    } else {
        None
    }
}

pub fn is_bucket_in_range(bucket: usize) -> (result: bool)
    ensures
        result <==> (bucket >= 4 && bucket <= 31),
{
    bucket >= 4 && bucket <= 31
}

fn test_bucket_calculation() {
    let size1 = order_to_size(10);

    let bucket_size = bucket_to_size(10);

    let valid = is_valid_bucket(10);
    let invalid = is_valid_bucket(MAX_ORDER);

    let aligned = size_is_bucket_aligned(100, 7);

    let max_bucket = get_max_bucket();
    let min_bucket = get_min_bucket();
    let min_size = get_min_block_size();

    let calc1 = calculate_bucket_size(5);
    let calc2 = calculate_bucket_size(2);
    let calc3 = calculate_bucket_size(70);

    let in_range1 = is_bucket_in_range(10);
    let in_range2 = is_bucket_in_range(100);
}

} // verus!

fn main() {
    test_bucket_calculation();
}
