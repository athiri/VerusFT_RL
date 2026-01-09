use vstd::prelude::*;

fn main() {}
verus! {

fn count_positive(a: &Vec<i32>) -> (count: usize)
    ensures
        count <= a.len(),
{
    let mut count: usize = 0;
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            count <= n,
        decreases a.len() - n,
    {
        if a[n] > 0 {
            count = count + 1;
        }
        n = n + 1;
    }
    count
}

} // verus!
