use vstd::prelude::*;
fn main() {}

verus!{

// Initialize array with zeros and verify sum is zero
pub fn array_zeros_sum(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
    requires
        N > 0,
        old(a).len() == N,
        old(sum).len() == 1,
    ensures
        sum[0] == 0,
        forall |k: int| 0 <= k < N ==> a[k] == 0,
{
    sum.set(0, 0);
    let mut i: usize = 0;

    // Initialize array with zeros using modulo
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            forall |k: int| 0 <= k < i ==> a[k] == 0,
        decreases N - i,
    {
        a.set(i, (i % 1) as i32);
        i = i + 1;
    }

    // Compute sum (should remain 0)
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            sum.len() == 1,
            sum[0] == 0,
            forall |k: int| 0 <= k < N ==> a[k] == 0,
        decreases N - i,
    {
        sum.set(0, sum[0] + a[i]);
        i = i + 1;
    }
}

}
