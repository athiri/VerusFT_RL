use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if all elements in a list are positive.

    assert!(all_positive(&vec![1, 2, 3, 4, 5]));
    assert!(!all_positive(&vec![1, -2, 3, 4, 5]));
    assert!(all_positive(&vec![10, 20, 30]));
}

verus! {

fn all_positive(nums: &Vec<i32>) -> (result: bool)
    ensures
        result == (forall|i: int| 0 <= i < nums.len() ==> nums[i] > 0),
{
    let mut index = 0;
    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> nums[k] > 0,
        decreases nums.len() - index,
    {
        if nums[index] <= 0 {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
