#[allow(unused_imports)]
use vstd::prelude::*;
fn main() {}

verus! {

fn find_min(nums: Vec<i32>) -> (min: i32)
    requires
        nums.len() > 0,
    ensures
        forall|i: int| 0 <= i < nums@.len() ==> nums@[i] >= min,
        exists|i: int| 0 <= i < nums@.len() && nums@[i] == min,
{
    let mut min = nums[0];
    let mut i = 1;
    while i < nums.len()
        invariant
            1 <= i <= nums.len(),
            forall|k: int| 0 <= k < i ==> nums@[k] >= min,
            exists|k: int| 0 <= k < i && nums@[k] == min,
        decreases nums.len() - i,
    {
        if nums[i] < min {
            min = nums[i];
        }
        i += 1;
    }
    min
}

} // verus!
