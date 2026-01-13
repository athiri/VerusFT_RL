// Variation 9: Min/Max with Specifications
// From: source/verismo/src/tspec_e/math/minmax.rs
// Demonstrates: Conditional logic, simple specifications

use vstd::prelude::*;

verus! {

pub open spec fn spec_min(x: int, y: int) -> int {
    if x < y { x } else { y }
}

pub open spec fn spec_max(x: int, y: int) -> int {
    if x > y { x } else { y }
}

pub fn min(x: u64, y: u64) -> (result: u64)
    ensures
        result as int == spec_min(x as int, y as int),
{
    if x < y { x } else { y }
}

pub fn max(x: u64, y: u64) -> (result: u64)
    ensures
        result as int == spec_max(x as int, y as int),
{
    if x > y { x } else { y }
}

pub fn clamp(value: u64, min_val: u64, max_val: u64) -> (result: u64)
    requires
        min_val <= max_val,
    ensures
        result >= min_val,
        result <= max_val,
{
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

fn test_min_max() {
    let m1 = min(5, 10);
    assert(m1 == 5);

    let m2 = max(5, 10);
    assert(m2 == 10);

    let c = clamp(15, 0, 10);
    assert(c >= 0 && c <= 10);
}

} // verus!

fn main() {
    test_min_max();
}
