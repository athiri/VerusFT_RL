use vstd::prelude::*;
fn main() {}

verus!{

// Accumulate counter to sum, then fill array
pub fn counter_accumulate(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
    requires
        N > 0,
        N < 1000,
        old(a).len() == N,
        old(sum).len() == 1,
    ensures
        sum[0] == N,
        forall |k: int| 0 <= k < N ==> a[k] == N,
{
    sum.set(0, 0);
    let mut i: usize = 0;

    // Accumulate counter
    while i < N as usize
        invariant
            i <= N,
            sum.len() == 1,
            sum[0] == i,
        decreases N - i,
    {
        sum.set(0, sum[0] + 1);
        i = i + 1;
    }

    // Fill array with sum value
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            sum.len() == 1,
            sum[0] == N,
            forall |k: int| 0 <= k < i ==> a[k] == sum[0],
        decreases N - i,
    {
        a.set(i, sum[0]);
        i = i + 1;
    }
}

}
