use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if a list contains a specific value.

    assert!(contains_value(&vec![10, 20, 30, 40], 30));
    assert!(!contains_value(&vec![1, 2, 3, 4], 5));
    assert!(contains_value(&vec![7, 8, 9, 10], 7));
}

verus! {

fn contains_value(nums: &Vec<i32>, target: i32) -> (result: bool)
    ensures
        result == (exists|i: int| 0 <= i < nums.len() && nums[i] == target),
{
    let mut index = 0;
    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> nums[k] != target,
        decreases nums.len() - index,
    {
        if nums[index] == target {
            return true;
        }
        index += 1;
    }
    false
}

} // verus!
