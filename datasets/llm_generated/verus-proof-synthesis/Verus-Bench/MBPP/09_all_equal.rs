use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if all elements in a list are equal.

    assert!(all_equal(&vec![5, 5, 5, 5]));
    assert!(!all_equal(&vec![5, 5, 6, 5]));
    assert!(all_equal(&vec![1, 1, 1]));
}

verus! {

fn all_equal(nums: &Vec<i32>) -> (result: bool)
    requires
        nums.len() > 0,
    ensures
        result == (forall|i: int| 0 <= i < nums.len() ==> #[trigger] nums[i] == nums[0]),
{
    let first = nums[0];
    let mut index = 1;

    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            first == nums[0],
            forall|k: int| 0 <= k < index ==> nums[k] == first,
        decreases nums.len() - index,
    {
        if nums[index] != first {
            assert(nums[index as int] != nums[0]);
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
