// Variation 2: Array Reverse with Loop Invariants
// Trimmed from: source/verismo/src/tspec_e/array/array_utils.rs
// Demonstrates: Loop invariants, two-pointer technique, range specifications

use vstd::prelude::*;

verus! {

#[verifier(inline)]
pub open spec fn spec_swap<T>(s: Seq<T>, i: int, j: int) -> Seq<T> {
    s.update(i, s[j]).update(j, s[i])
}

// In-place array reversal using two pointers
pub fn reverse<T: Copy>(arr: &mut Vec<T>, start: usize, end: usize)
    requires
        0 <= start <= end <= old(arr).len(),
    ensures
        arr.len() == old(arr).len(),
        // Elements in range are reversed
        forall|k: int| start <= k < end
            ==> arr[k] === old(arr)[end as int + start as int - 1 - k],
        // Elements outside range unchanged
        forall|k: int| (0 <= k < start || end <= k < arr.len())
            ==> arr[k] === old(arr)[k],
{
    if end < 1 || start >= end - 1 {
        return;
    }

    let mut i: usize = start;
    let mut j: usize = end - 1;

    while i < j
        invariant
            i <= j + 1,
            start <= i,
            j <= end - 1,
            j < arr.len(),
            i < arr.len(),
            // Symmetry: equal distance from boundaries
            end - 1 - j == i - start,
            0 <= start < end <= arr.len(),
            arr.len() == old(arr).len(),
            // Already reversed portion
            forall|k: int| (start <= k < i || j < k < end)
                ==> arr[k] === old(arr)[end as int + start as int - 1 - k],
            // Unreversed middle portion unchanged
            forall|k: int| (i <= k <= j || 0 <= k < start || end <= k < arr.len())
                ==> arr[k] === old(arr)[k],
        decreases j
    {
        // Swap elements and move pointers inward
        let temp = arr[i];
        arr.set(i, arr[j]);
        arr.set(j, temp);

        proof {
            // Help Verus understand the decreases: (j-1) - (i+1) = j - i - 2
            assert(i < j);  // Loop condition
        }

        i = i + 1;
        j = j - 1;
    }
}

// Test function
fn test_reverse() {
    let mut v = vec![1, 2, 3, 4, 5];
    reverse(&mut v, 1, 4); // Reverse [2, 3, 4] to [4, 3, 2]
    assert(v[0] == 1);
    assert(v[1] == 4);
    assert(v[2] == 3);
    assert(v[3] == 2);
    assert(v[4] == 5);
}

} // verus!

fn main() {
    test_reverse();
}
