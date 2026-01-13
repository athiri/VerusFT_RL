use vstd::prelude::*;

verus! {

pub open spec fn triangle(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}


pub proof fn triangle_is_monotonic(i: nat, j: nat)
    ensures i <= j ==> triangle(i) <= triangle(j)
    decreases j,
{
    if j == 0 {
    } else {
        triangle_is_monotonic(i, (j - 1) as nat);
    }
}

} // verus!