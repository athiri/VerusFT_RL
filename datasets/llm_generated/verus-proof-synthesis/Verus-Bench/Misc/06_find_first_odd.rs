use vstd::prelude::*;
fn main() {}

verus! {

fn find_first_odd(nums: &Vec<u64>) -> (index: usize)
    requires
        exists|i: int| 0 <= i < nums.len() && nums[i] % 2 == 1,
    ensures
        index < nums.len(),
        nums[index as int] % 2 == 1,
        forall|k: int| 0 <= k < index ==> #[trigger] nums[k] % 2 != 1,
{
    let mut i: usize = 0;

    while i < nums.len()
        invariant
            0 <= i <= nums.len(),
            forall|k: int| 0 <= k < i ==> #[trigger] nums[k] % 2 != 1,
        decreases nums.len() - i,
    {
        if nums[i] % 2 == 1 {
            return i;
        }
        i = i + 1;
    }
    i
}

} // verus!
