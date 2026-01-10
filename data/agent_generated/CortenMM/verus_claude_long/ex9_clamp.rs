// Source: verification/common/src/helpers/math.rs
// Concept: Clamping a value to a range
// This example demonstrates:
// - A spec function defining clamped value mathematically
// - An executable function clamping with conditional logic
// - The ensures clause guarantees the result is within bounds

use vstd::prelude::*;

verus! {

/// Spec function: clamp a value to the range [min, max]
/// Returns min if val < min, max if val > max, otherwise val
pub open spec fn clamp_spec(val: u64, min: u64, max: u64) -> u64
    recommends
        min <= max,
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Executable function: clamp a value to the range [min, max]
/// Ensures the result is always between min and max inclusive
pub fn clamp(val: u64, min: u64, max: u64) -> (res: u64)
    requires
        min <= max,
    ensures
        res == clamp_spec(val, min, max),
        min <= res <= max,
        val < min ==> res == min,
        val > max ==> res == max,
        min <= val <= max ==> res == val,
{
    let result: u64;
    
    if val < min {
        result = min;
        assert(result == min);
        assert(min <= result);
        assert(result <= max);
    } else if val > max {
        result = max;
        assert(result == max);
        assert(min <= result);
        assert(result <= max);
    } else {
        result = val;
        assert(result == val);
        assert(min <= result);
        assert(result <= max);
    }
    
    result
}

fn main() {
    let r1 = clamp(50, 0, 100);
    assert(r1 == 50);
    
    let r2 = clamp(150, 0, 100);
    assert(r2 == 100);
    
    let r3 = clamp(0, 10, 100);
    assert(r3 == 10);
    
    let r4 = clamp(75, 50, 80);
    assert(r4 == 75);
}

} // verus!

