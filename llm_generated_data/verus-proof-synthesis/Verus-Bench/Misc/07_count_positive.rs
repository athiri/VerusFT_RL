#[allow(unused_imports)]
use vstd::prelude::*;
fn main() {}

verus! {

fn count_positive(nums: Vec<i32>) -> (count: usize)
    requires
        nums.len() < 1000,
{
    let mut count: usize = 0;
    let mut i: usize = 0;

    while i < nums.len()
        invariant
            0 <= i <= nums.len(),
            count <= i,
            nums.len() < 1000,
        decreases nums.len() - i,
    {
        if nums[i] > 0 {
            count = count + 1;
        }
        i = i + 1;
    }
    count
}

} // verus!
