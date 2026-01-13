use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if all elements in a list are even.

    assert!(all_even(&vec![2, 4, 6, 8]));
    assert!(!all_even(&vec![2, 3, 4, 6]));
    assert!(all_even(&vec![10, 20, 30, 40]));
}

verus! {

fn all_even(nums: &Vec<i32>) -> (result: bool)
    ensures
        result == (forall|i: int| 0 <= i < nums.len() ==> (nums[i] % 2) == 0),
{
    let mut index = 0;
    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> (nums[k] % 2) == 0,
        decreases nums.len() - index,
    {
        if (nums[index] % 2) != 0 {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
