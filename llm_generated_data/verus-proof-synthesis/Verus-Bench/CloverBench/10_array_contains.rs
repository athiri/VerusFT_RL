use vstd::prelude::*;

fn main() {}
verus! {

fn array_contains(a: &Vec<i32>, target: i32) -> (result: bool)
    ensures
        result <==> exists|i: int| 0 <= i < a.len() && a[i] == target,
{
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            forall|i: int| 0 <= i < n ==> a[i] != target,
        decreases a.len() - n,
    {
        if a[n] == target {
            return true;
        }
        n = n + 1;
    }
    false
}

} // verus!
