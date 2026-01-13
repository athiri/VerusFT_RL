// Variation 4: Array Partition with Loop Invariants
// Simplified version without function pointers
// Demonstrates: Loop invariants, partitioning

use vstd::prelude::*;

verus! {

// Simplified partition function for integers
// Partitions array around the last element
pub fn partition(arr: &mut Vec<i32>, start: usize, end: usize) -> (result: usize)
    requires
        start < end,
        end <= old(arr).len(),
    ensures
        arr.len() == old(arr).len(),
        start <= result < end,
{
    if start >= end || start + 1 >= end {
        return start;
    }

    let pivot = arr[end - 1];
    let mut i = start;
    let mut j = start;

    // Partition loop: i tracks "less than" boundary, j scans elements
    while j < end - 1
        invariant
            start <= i <= j,
            j < end,
            end <= arr.len(),
            arr.len() == old(arr).len(),
            // All elements before i are less than pivot
            forall|k: int| start <= k < i ==> arr[k] < pivot,
            // All elements between i and j are >= pivot
            forall|k: int| i <= k < j ==> arr[k] >= pivot,
        decreases end - j - 1
    {
        // If current element is less than pivot, swap it to the "less than" region
        if arr[j] < pivot {
            let temp = arr[i];
            arr.set(i, arr[j]);
            arr.set(j, temp);
            i = i + 1;
        }
        j = j + 1;
    }

    // Place pivot at boundary
    let temp = arr[i];
    arr.set(i, arr[end - 1]);
    arr.set(end - 1, temp);

    i
}

// Test function
fn test_partition() {
    let mut v = vec![3, 7, 8, 5, 2, 1, 9, 6, 4];
    let pivot_pos = partition(&mut v, 0, 9);
    // After partition, elements less than v[pivot_pos] are before it
    // Elements >= v[pivot_pos] are after it
    assert(pivot_pos < 9);
}

} // verus!

fn main() {
    test_partition();
}
