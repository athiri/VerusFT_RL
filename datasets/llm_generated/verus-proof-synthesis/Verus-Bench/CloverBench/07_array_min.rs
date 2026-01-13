use vstd::prelude::*;

fn main() {}
verus! {

fn array_min(a: &Vec<i32>) -> (min: i32)
    requires
        a.len() > 0,
    ensures
        forall|i: int| 0 <= i < a.len() ==> min <= a[i],
        exists|i: int| 0 <= i < a.len() && min == a[i],
{
    let mut min = a[0];
    let mut n: usize = 1;
    while n < a.len()
        invariant
            n <= a.len(),
            forall|i: int| 0 <= i < n ==> min <= a[i],
            exists|i: int| 0 <= i < n && min == a[i],
        decreases a.len() - n,
    {
        if a[n] < min {
            min = a[n];
        }
        n = n + 1;
    }
    min
}

} // verus!
