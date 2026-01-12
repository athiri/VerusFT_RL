use vstd::prelude::*;
fn main() {}

verus!{

// Initialize two arrays and sum both
pub fn two_arrays_sum(a: &mut Vec<i32>, b: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
    requires
        N > 0,
        N < 1000,
        old(a).len() == N,
        old(b).len() == N,
        old(sum).len() == 1,
    ensures
        sum[0] == 2 * N,
        forall |k: int| 0 <= k < N ==> a[k] == 1,
        forall |k: int| 0 <= k < N ==> b[k] == 1,
{
    sum.set(0, 0);
    let mut i: usize = 0;

    // Initialize array a with ones
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

    // Initialize array b with ones
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            b.len() == N,
            forall |k: int| 0 <= k < i ==> b[k] == 1,
        decreases N - i,
    {
        b.set(i, 1);
        i = i + 1;
    }

    // Sum array a
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            sum.len() == 1,
            sum[0] == i,
            forall |k: int| 0 <= k < N ==> a[k] == 1,
            N < 1000,
        decreases N - i,
    {
        sum.set(0, sum[0] + a[i]);
        i = i + 1;
    }

    // Sum array b
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            b.len() == N,
            sum.len() == 1,
            sum[0] == N + i,
            forall |k: int| 0 <= k < N ==> b[k] == 1,
            N < 1000,
        decreases N - i,
    {
        sum.set(0, sum[0] + b[i]);
        i = i + 1;
    }
}

}
