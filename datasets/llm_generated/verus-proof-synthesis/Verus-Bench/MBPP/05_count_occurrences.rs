use vstd::prelude::*;

fn main() {
    // Write a function in Rust to count occurrences of a value in a list.

    assert_eq!(count_occurrences(&vec![1, 2, 3, 2, 4, 2], 2), 3);
    assert_eq!(count_occurrences(&vec![1, 1, 1, 1], 1), 4);
    assert_eq!(count_occurrences(&vec![5, 6, 7, 8], 9), 0);
}

verus! {

fn count_occurrences(nums: &Vec<i32>, target: i32) -> (count: usize)
    requires
        nums.len() < 100,
{
    let mut count: usize = 0;
    let mut index: usize = 0;

    while index < nums.len()
        invariant
            0 <= index <= nums.len(),
            count <= index,
            nums.len() < 100,
        decreases nums.len() - index,
    {
        if nums[index] == target {
            count = count + 1;
        }
        index = index + 1;
    }
    count
}

} // verus!
