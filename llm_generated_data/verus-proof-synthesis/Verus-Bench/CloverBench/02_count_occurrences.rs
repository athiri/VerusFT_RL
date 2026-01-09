use vstd::prelude::*;

fn main() {}
verus! {

fn count_occurrences(a: &Vec<i32>, target: i32) -> (count: usize)
    ensures
        count <= a.len(),
{
    let mut count: usize = 0;
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            count <= n,
            forall|i: int| 0 <= i < n && a[i] == target ==> count > 0,
        decreases a.len() - n,
    {
        if a[n] == target {
            count = count + 1;
        }
        n = n + 1;
    }
    count
}

} // verus!
