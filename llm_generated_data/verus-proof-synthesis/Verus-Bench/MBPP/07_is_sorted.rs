use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if a list is sorted in ascending order.

    assert!(is_sorted(&vec![1, 2, 3, 4, 5]));
    assert!(!is_sorted(&vec![1, 3, 2, 4, 5]));
    assert!(is_sorted(&vec![10, 20, 30]));
}

verus! {

fn is_sorted(nums: &Vec<i32>) -> (result: bool)
    ensures
        result == (forall|i: int, j: int| 0 <= i < j < nums.len() ==> nums[i] <= nums[j]),
{
    if nums.len() <= 1 {
        return true;
    }

    let mut index = 1;
    while index < nums.len()
        invariant
            1 <= index <= nums.len(),
            forall|i: int, j: int| 0 <= i < j < index ==> nums[i] <= nums[j],
        decreases nums.len() - index,
    {
        if nums[index - 1] > nums[index] {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
