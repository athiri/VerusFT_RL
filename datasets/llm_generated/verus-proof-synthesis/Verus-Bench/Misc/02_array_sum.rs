#[allow(unused_imports)]
use vstd::prelude::*;
fn main() {}

verus! {

fn array_sum(nums: Vec<u32>) -> (sum: u64)
    requires
        nums.len() < 100,
        forall|i: int| 0 <= i < nums.len() ==> #[trigger] nums@[i] <= 100,
{
    let mut sum: u64 = 0;
    let mut i: usize = 0;

    while i < nums.len()
        invariant
            0 <= i <= nums.len(),
            nums.len() < 100,
            sum <= i * 100,
            forall|k: int| 0 <= k < nums.len() ==> #[trigger] nums@[k] <= 100,
        decreases nums.len() - i,
    {
        sum = sum + (nums[i] as u64);
        i += 1;
    }
    sum
}

} // verus!
