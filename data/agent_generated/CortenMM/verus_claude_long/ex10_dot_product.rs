// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Computing dot product of two vectors
// This example demonstrates:
// - A spec function defining dot product over sequences
// - An executable function computing dot product with loop
// - Ensures the result matches the mathematical definition

use vstd::prelude::*;

verus! {

/// Spec function: compute dot product of first n elements of two sequences
/// Base case: dot product of 0 elements is 0
/// Recursive case: add product of current elements to dot product of rest
pub open spec fn dot_product_spec(a: Seq<u64>, b: Seq<u64>, n: int) -> int
    recommends
        0 <= n <= a.len(),
        0 <= n <= b.len(),
    decreases n,
{
    if n <= 0 {
        0
    } else {
        dot_product_spec(a, b, n - 1) + (a[n - 1] as int) * (b[n - 1] as int)
    }
}

/// Executable function: compute dot product of two equal-length vectors
/// Uses loop to accumulate sum of element-wise products
pub fn dot_product(a: &Vec<u64>, b: &Vec<u64>) -> (res: u64)
    requires
        a.len() == b.len(),
        a.len() <= 100,
        forall|i: int| 0 <= i < a.len() ==> a[i] <= 100,
        forall|i: int| 0 <= i < b.len() ==> b[i] <= 100,
    ensures
        res as int == dot_product_spec(a@, b@, a.len() as int),
{
    let len: usize = a.len();
    let mut sum: u64 = 0;
    let mut idx: usize = 0;
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == a.len(),
            len == b.len(),
            len <= 100,
            forall|i: int| 0 <= i < a.len() ==> a[i] <= 100,
            forall|i: int| 0 <= i < b.len() ==> b[i] <= 100,
            sum as int == dot_product_spec(a@, b@, idx as int),
            sum <= idx as u64 * 10000,
        decreases len - idx,
    {
        let a_val: u64 = a[idx];
        let b_val: u64 = b[idx];
        
        assert(a_val <= 100);
        assert(b_val <= 100);
        assert(a_val * b_val <= 10000) by (nonlinear_arith)
            requires a_val <= 100, b_val <= 100;
        
        let prod: u64 = a_val * b_val;
        
        assert(sum + prod <= (idx as u64 + 1) * 10000) by (nonlinear_arith)
            requires
                sum <= idx as u64 * 10000,
                prod <= 10000,
        ;
        
        sum = sum + prod;
        idx = idx + 1;
    }
    
    sum
}

fn main() {
    let mut a: Vec<u64> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    
    let mut b: Vec<u64> = Vec::new();
    b.push(4);
    b.push(5);
    b.push(6);
    
    let result = dot_product(&a, &b);
}

} // verus!
