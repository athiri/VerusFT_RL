// Source: verification/vstd_extra/src/seq_extra.rs
// Concept: Checking if all elements in a sequence are positive
// This example demonstrates:
// - A spec function defining all-positive property
// - An executable function checking with early exit
// - The ensures clause guarantees correct boolean result

use vstd::prelude::*;

verus! {

/// Spec function: check if all elements in sequence are positive (> 0)
/// Uses universal quantifier over all indices
pub open spec fn all_positive(s: Seq<u64>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> s[i] > 0
}

/// Executable function: check if all elements in a vector are positive
/// Returns true if all elements are > 0, false otherwise
pub fn check_all_positive(v: &Vec<u64>) -> (res: bool)
    ensures
        res == all_positive(v@),
{
    let len: usize = v.len();
    let mut idx: usize = 0;
    
    while idx < len
        invariant
            0 <= idx <= len,
            len == v.len(),
            forall|j: int| 0 <= j < idx ==> v[j] > 0,
        decreases len - idx,
    {
        let val: u64 = v[idx];
        if val == 0 {
            assert(!all_positive(v@)) by {
                assert(v[idx as int] == 0);
            };
            return false;
        }
        idx = idx + 1;
    }
    
    assert(all_positive(v@));
    true
}

fn main() {
    let mut v1: Vec<u64> = Vec::new();
    v1.push(10);
    v1.push(20);
    v1.push(30);
    
    let result1 = check_all_positive(&v1);
    assert(result1 == true);
    
    let mut v2: Vec<u64> = Vec::new();
    v2.push(10);
    v2.push(0);
    v2.push(30);
    
    let result2 = check_all_positive(&v2);
}

} // verus!

