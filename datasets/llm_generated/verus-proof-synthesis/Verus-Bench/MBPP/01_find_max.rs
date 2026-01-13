use vstd::prelude::*;

fn main() {
    // Write a function in Rust to find maximum number in a list.

    assert_eq!(find_max(&vec![10, 20, 1, 45, 99]), 99);
    assert_eq!(find_max(&vec![1, 2, 3]), 3);
    assert_eq!(find_max(&vec![45, 46, 50, 60]), 60);
}

verus! {

fn find_max(nums: &Vec<i32>) -> (max: i32)
    requires
        nums.len() > 0,
    ensures
        forall|i: int| 0 <= i < nums.len() ==> max >= nums[i],
        exists|i: int| 0 <= i < nums.len() && max == nums[i],
{
    let mut max = nums[0];
    let mut index = 1;

    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            forall|k: int| 0 <= k < index ==> max >= nums[k],
            exists|k: int| 0 <= k < index && max == nums[k],
        decreases nums.len() - index,
    {
        if nums[index] > max {
            max = nums[index];
        }
        index += 1;
    }
    max
}

} // verus!
