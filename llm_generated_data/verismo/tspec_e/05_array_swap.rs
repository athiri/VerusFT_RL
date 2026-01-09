// Variation 1: Simple Array Swap
// Trimmed from: source/verismo/src/tspec_e/array/array_utils.rs
// Demonstrates: Basic requires/ensures, proof blocks, simple loop-free verification

use vstd::prelude::*;

verus! {

// Spec function for swapping two elements in a sequence
#[verifier(inline)]
pub open spec fn spec_swap<T>(s: Seq<T>, i: int, j: int) -> Seq<T> {
    s.update(i, s[j]).update(j, s[i])
}

// Simple array swap with verification
pub fn swap<T: Copy>(arr: &mut Vec<T>, i: usize, j: usize)
    requires
        i < old(arr).len(),
        j < old(arr).len(),
    ensures
        arr.len() == old(arr).len(),
        // Elements are swapped
        arr[i as int] === old(arr)[j as int],
        arr[j as int] === old(arr)[i as int],
        // All other elements unchanged
        forall|k: int| 0 <= k < arr.len() && k != i as int && k != j as int
            ==> arr[k] === old(arr)[k],
        // Sequence-level specification
        arr@ =~~= spec_swap(old(arr)@, i as int, j as int),
{
    // Early return if swapping same element
    if i == j {
        return;
    }

    let x1 = arr[i];
    let x2 = arr[j];

    proof {
        assert(x1 === old(arr)@[i as int]);
        assert(x2 === old(arr)@[j as int]);
    }

    arr.set(j, x1);
    arr.set(i, x2);

    proof {
        assert forall|k: int|
            0 <= k < arr.len() && k != i as int && k != j as int
            implies arr[k] === old(arr)[k] by {}
    }
}

// Test function
fn test_swap() {
    let mut v = vec![1, 2, 3, 4, 5];
    swap(&mut v, 0, 4);
    assert(v[0] == 5);
    assert(v[4] == 1);
    assert(v[2] == 3); // Middle element unchanged
}

} // verus!

fn main() {
    test_swap();
}
