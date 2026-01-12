use vstd::prelude::*;

fn main() {}
verus! {

fn array_max(a: &Vec<i32>) -> (max: i32)
    requires
        a.len() > 0,
    ensures
        forall|i: int| 0 <= i < a.len() ==> max >= a[i],
        exists|i: int| 0 <= i < a.len() && max == a[i],
{
    let mut max = a[0];
    let mut n: usize = 1;
    while n < a.len()
        invariant
            n <= a.len(),
            forall|i: int| 0 <= i < n ==> max >= a[i],
            exists|i: int| 0 <= i < n && max == a[i],
        decreases a.len() - n,
    {
        if a[n] > max {
            max = a[n];
        }
        n = n + 1;
    }
    max
}

} // verus!
