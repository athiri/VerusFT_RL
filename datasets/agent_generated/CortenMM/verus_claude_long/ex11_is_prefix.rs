// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Checking if one sequence is a prefix of another
// This example demonstrates:
// - A spec function defining the prefix relationship
// - An executable function checking prefix with loop
// - The ensures clause guarantees correct boolean result

use vstd::prelude::*;

verus! {

/// Spec function: check if sequence a is a prefix of sequence b
/// a is prefix of b if a.len() <= b.len() and all elements match
pub open spec fn is_prefix(a: Seq<u64>, b: Seq<u64>) -> bool {
    a.len() <= b.len() && forall|i: int| 0 <= i < a.len() ==> a[i] == b[i]
}

/// Executable function: check if vector a is a prefix of vector b
/// Returns true if all elements of a match the corresponding elements of b
pub fn check_is_prefix(a: &Vec<u64>, b: &Vec<u64>) -> (res: bool)
    ensures
        res == is_prefix(a@, b@),
{
    let a_len: usize = a.len();
    let b_len: usize = b.len();
    
    if a_len > b_len {
        assert(!is_prefix(a@, b@));
        return false;
    }
    
    let mut idx: usize = 0;
    
    while idx < a_len
        invariant
            0 <= idx <= a_len,
            a_len == a.len(),
            b_len == b.len(),
            a_len <= b_len,
            forall|j: int| 0 <= j < idx ==> a[j] == b[j],
        decreases a_len - idx,
    {
        let a_val: u64 = a[idx];
        let b_val: u64 = b[idx];
        
        if a_val != b_val {
            assert(!is_prefix(a@, b@)) by {
                assert(a[idx as int] != b[idx as int]);
            };
            return false;
        }
        idx = idx + 1;
    }
    
    assert(is_prefix(a@, b@));
    true
}

fn main() {
    let mut a: Vec<u64> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    
    let mut b: Vec<u64> = Vec::new();
    b.push(1);
    b.push(2);
    b.push(3);
    b.push(4);
    b.push(5);
    
    let result1 = check_is_prefix(&a, &b);
    assert(result1 == true);
    
    let mut c: Vec<u64> = Vec::new();
    c.push(1);
    c.push(9);
    
    let result2 = check_is_prefix(&c, &b);
}

} // verus!

