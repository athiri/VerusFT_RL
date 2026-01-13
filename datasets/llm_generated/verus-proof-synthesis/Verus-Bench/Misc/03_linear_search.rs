#[allow(unused_imports)]
use vstd::prelude::*;
fn main() {}

verus! {

fn linear_search(nums: Vec<i32>, target: i32) -> (index: i32)
    requires
        nums@.len() < 0x8000_0000,
    ensures
        index < nums@.len(),
        index >= 0 ==> nums@[index as int] == target,
        index >= 0 ==> forall|i: int| 0 <= i < index as int ==> #[trigger] nums@[i] != target,
        index < 0 ==> forall|i: int| 0 <= i < nums@.len() ==> #[trigger] nums@[i] != target,
{
    let mut i = 0;
    while i < nums.len()
        invariant
            0 <= i <= nums@.len(),
            forall|k: int| 0 <= k < i ==> #[trigger] nums@[k] != target,
        ensures
            0 <= i < nums@.len() ==> (#[trigger] nums@[i as int]) == target,
        decreases nums.len() - i,
    {
        if nums[i] == target {
            break;
        }
        i = i + 1;
    }
    if i == nums.len() {
        -1
    } else {
        i as i32
    }
}

} // verus!
