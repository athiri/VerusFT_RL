use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if a list contains any negative numbers.

    assert!(contains_negative(&vec![1, -2, 3, 4]));
    assert!(!contains_negative(&vec![1, 2, 3, 4]));
    assert!(contains_negative(&vec![0, -1, 5]));
}

verus! {

fn contains_negative(nums: &Vec<i32>) -> (result: bool)
    ensures
        result == (exists|i: int| 0 <= i < nums.len() && nums[i] < 0),
{
    let mut index = 0;
    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> nums[k] >= 0,
        decreases nums.len() - index,
    {
        if nums[index] < 0 {
            return true;
        }
        index += 1;
    }
    false
}

} // verus!
