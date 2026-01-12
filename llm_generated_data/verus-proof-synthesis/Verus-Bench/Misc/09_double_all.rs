use vstd::prelude::*;
fn main() {}

verus! {

fn double_all(nums: &mut Vec<u32>)
    requires
        forall|k: int| 0 <= k < old(nums).len() ==> #[trigger] old(nums)[k] <= 0x7FFF_FFFF,
    ensures
        nums@.len() == old(nums)@.len(),
        forall|k: int| 0 <= k < nums.len() ==> #[trigger] nums@[k] == old(nums)@[k] * 2,
{
    let mut i: usize = 0;
    let len: usize = nums.len();

    while i < len
        invariant
            i <= len,
            len == nums.len(),
            forall|k: int| 0 <= k < i ==> #[trigger] nums[k] == old(nums)[k] * 2,
            forall|k: int| i <= k < len ==> nums[k] == old(nums)[k],
            forall|k: int| 0 <= k < len ==> old(nums)[k] <= 0x7FFF_FFFF,
        decreases len - i,
    {
        nums.set(i, nums[i] * 2);
        i = i + 1;
    }
}

} // verus!
