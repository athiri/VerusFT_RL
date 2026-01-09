use vstd::prelude::*;
fn main() {}

verus! {

fn increment_all(nums: &mut Vec<i32>)
    requires
        forall|k: int| 0 <= k < old(nums).len() ==> #[trigger] old(nums)[k] <= 0x7FFF_FFFE,
    ensures
        nums@.len() == old(nums)@.len(),
        forall|k: int| 0 <= k < nums.len() ==> #[trigger] nums@[k] == old(nums)@[k] + 1,
{
    let mut i: usize = 0;
    let len: usize = nums.len();

    while i < len
        invariant
            i <= len,
            len == nums.len(),
            forall|k: int| 0 <= k < i ==> #[trigger] nums[k] == old(nums)[k] + 1,
            forall|k: int| i <= k < len ==> nums[k] == old(nums)[k],
            forall|k: int| 0 <= k < len ==> old(nums)[k] <= 0x7FFF_FFFE,
        decreases len - i,
    {
        nums.set(i, nums[i] + 1);
        i = i + 1;
    }
}

} // verus!
