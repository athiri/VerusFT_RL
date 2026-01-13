use vstd::prelude::*;
fn main() {}

verus! {

fn reverse(nums: &mut Vec<u64>)
    ensures
        nums.len() == old(nums).len(),
        forall|i: int| 0 <= i < old(nums).len() ==> nums[i] == old(nums)[old(nums).len() - i - 1],
{
    let length = nums.len();
    let mut i: usize = 0;

    while i < length / 2
        invariant
            0 <= i <= length / 2,
            nums.len() == old(nums).len(),
            nums.len() == length,
            forall|k: int| 0 <= k < i ==> nums[k] == old(nums)[length - k - 1],
            forall|k: int| 0 <= k < i ==> nums[length - k - 1] == old(nums)[k],
            forall|k: int| i <= k < length - i ==> nums[k] == old(nums)[k],
        decreases length / 2 - i,
    {
        let left_val = nums[i];
        let right_val = nums[length - 1 - i];
        nums.set(i, right_val);
        nums.set(length - 1 - i, left_val);
        i = i + 1;
    }
}

} // verus!
