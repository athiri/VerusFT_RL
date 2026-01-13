use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if all elements in a list are zero.

    assert!(all_zeros(&vec![0, 0, 0, 0]));
    assert!(!all_zeros(&vec![0, 1, 0, 0]));
    assert!(all_zeros(&vec![0, 0]));
}

verus! {

fn all_zeros(nums: &Vec<i32>) -> (result: bool)
    ensures
        result == (forall|i: int| 0 <= i < nums.len() ==> nums[i] == 0),
{
    let mut index = 0;
    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> nums[k] == 0,
        decreases nums.len() - index,
    {
        if nums[index] != 0 {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
