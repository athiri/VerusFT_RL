use vstd::prelude::*;
fn main() {}

verus!{

// Initialize array with ones, double each element, then sum
pub fn array_double(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
    requires
        N > 0,
        N < 1000,
        old(a).len() == N,
        old(sum).len() == 1,
    ensures
        sum[0] == 2 * N,
        forall |k: int| 0 <= k < N ==> a[k] == 2,
{
    sum.set(0, 0);
    let mut i: usize = 0;

    // Initialize array with ones
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            forall |k: int| 0 <= k < i ==> a[k] == 1,
        decreases N - i,
    {
        a.set(i, 1);
        i = i + 1;
    }

    // Double each element
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            forall |k: int| 0 <= k < i ==> a[k] == 2,
            forall |k: int| i <= k < N ==> a[k] == 1,
        decreases N - i,
    {
        a.set(i, a[i] + 1);
        i = i + 1;
    }

    // Compute sum
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            sum.len() == 1,
            sum[0] == 2 * i,
            forall |k: int| 0 <= k < N ==> a[k] == 2,
            N < 1000,
        decreases N - i,
    {
        sum.set(0, sum[0] + a[i]);
        i = i + 1;
    }
}

}
