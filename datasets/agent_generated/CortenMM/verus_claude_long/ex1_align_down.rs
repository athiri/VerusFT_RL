// Source: verification/common/src/helpers/align_ext.rs
// Concept: Memory alignment down to an alignment boundary
// This example demonstrates:
// - A spec function defining alignment mathematically (using division/multiplication)
// - An executable function computing alignment using arithmetic
// - The ensures clause connects the executable result to the spec definition

use vstd::prelude::*;

verus! {

/// Spec function: compute the aligned-down value mathematically
/// Given a value x and alignment, returns the largest multiple of align <= x
pub open spec fn align_down_spec(x: u64, align: u64) -> u64
    recommends
        align > 0,
{
    ((x / align) * align) as u64
}

/// Executable function: align x down to the nearest multiple of align
/// Uses division and multiplication to compute the aligned value
pub fn align_down(x: u64, align: u64) -> (res: u64)
    requires
        align > 0,
    ensures
        res == align_down_spec(x, align),
        res <= x,
        res % align == 0,
{
    let quotient: u64 = x / align;
    
    assert(quotient * align <= x) by {
        assert((x as int / align as int) * align as int <= x as int) by (nonlinear_arith)
            requires align as int > 0;
    };
    
    let product: u64 = quotient * align;
    let result: u64 = product;
    
    assert(result <= x) by {
        assert((x as int / align as int) * align as int <= x as int) by (nonlinear_arith)
            requires align as int > 0;
    };
    
    assert(result % align == 0) by {
        assert(((x as int / align as int) * align as int) % align as int == 0) by (nonlinear_arith)
            requires align as int > 0;
    };
    
    result
}

fn main() {
    let x1: u64 = 100;
    let align1: u64 = 16;
    let result1 = align_down(x1, align1);
    
    let x2: u64 = 4097;
    let align2: u64 = 4096;
    let result2 = align_down(x2, align2);
    
    let x3: u64 = 256;
    let align3: u64 = 64;
    let result3 = align_down(x3, align3);
    
    assert(result3 % 64 == 0);
}

} // verus!
