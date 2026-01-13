use vstd::prelude::*;
fn main() {}

verus!{

// Conditionally modify array elements
pub fn conditional_modify(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
    requires
        N > 0,
        N < 1000,
        old(a).len() == N,
        old(sum).len() == 1,
    ensures
        sum[0] == N,
        forall |k: int| 0 <= k < N ==> a[k] == 1,
{
    sum.set(0, 0);
    let mut i: usize = 0;

    // Initialize array with zeros
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            forall |k: int| 0 <= k < i ==> a[k] == 0,
        decreases N - i,
    {
        a.set(i, 0);
        i = i + 1;
    }

    // Conditionally set to 1 (always true condition)
    i = 0;
    while i < N as usize
        invariant
            i <= N,
            a.len() == N,
            forall |k: int| 0 <= k < i ==> a[k] == 1,
            forall |k: int| i <= k < N ==> a[k] == 0,
        decreases N - i,
    {
        if a[i] == 0 {
            a.set(i, 1);
        } else {
            a.set(i, 0);
        }
        i = i + 1;
    }

    // Sum elements
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
}

}
