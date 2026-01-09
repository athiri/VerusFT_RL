use vstd::prelude::*;

fn main() {}
verus! {

fn array_reverse(a: &Vec<i32>) -> (reversed: Vec<i32>)
    ensures
        reversed.len() == a.len(),
        forall|i: int| 0 <= i < a.len() ==> reversed[i] == a[a.len() - 1 - i],
{
    let mut reversed = Vec::with_capacity(a.len());
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            reversed.len() == n,
            forall|i: int| 0 <= i < n ==> reversed[i] == a[a.len() - 1 - i],
        decreases a.len() - n,
    {
        reversed.push(a[a.len() - 1 - n]);
        n = n + 1;
    }
    reversed
}

} // verus!
