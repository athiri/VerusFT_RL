// Variation 3: Array Memset with Simple Loop
// Trimmed from: source/verismo/src/tspec_e/array/array_utils.rs
// Demonstrates: Simple loop invariants, array initialization

use vstd::prelude::*;

verus! {

// Fill array with a single value
pub fn memset<T: Copy>(arr: &mut Vec<T>, elem: T)
    requires
        old(arr).len() > 0,
    ensures
        arr.len() == old(arr).len(),
        forall|j: int| 0 <= j < arr.len() ==> arr[j] === elem,
{
    let mut i = 0;

    while i < arr.len()
        invariant
            0 <= i <= arr.len(),
            arr.len() == old(arr).len(),
            // All elements before i are set to elem
            forall|j: int| 0 <= j < i ==> arr[j] === elem,
        decreases arr.len() - i
    {
        arr.set(i, elem);
        i = i + 1;
    }
}

// Variation: Fill only a range
pub fn memset_range<T: Copy>(arr: &mut Vec<T>, elem: T, start: usize, end: usize)
    requires
        0 <= start <= end <= old(arr).len(),
    ensures
        arr.len() == old(arr).len(),
        // Elements in range are set
        forall|j: int| start <= j < end ==> arr[j] === elem,
        // Elements outside range unchanged
        forall|j: int| (0 <= j < start || end <= j < arr.len())
            ==> arr[j] === old(arr)[j],
{
    let mut i = start;

    while i < end
        invariant
            start <= i <= end <= arr.len(),
            arr.len() == old(arr).len(),
            forall|j: int| start <= j < i ==> arr[j] === elem,
            forall|j: int| (0 <= j < start || end <= j < arr.len())
                ==> arr[j] === old(arr)[j],
        decreases end - i
    {
        arr.set(i, elem);
        i = i + 1;
    }
}

// Test function
fn test_memset() {
    let mut v = vec![1, 2, 3, 4, 5];
    memset(&mut v, 0);
    assert(v[0] == 0);
    assert(v[4] == 0);
}

fn test_memset_range() {
    let mut v = vec![1, 2, 3, 4, 5];
    memset_range(&mut v, 0, 1, 4);
    assert(v[0] == 1);  // Unchanged
    assert(v[1] == 0);
    assert(v[3] == 0);
    assert(v[4] == 5);  // Unchanged
}

} // verus!

fn main() {
    test_memset();
    test_memset_range();
}
