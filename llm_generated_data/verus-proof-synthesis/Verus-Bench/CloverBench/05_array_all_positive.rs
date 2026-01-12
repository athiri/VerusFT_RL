use vstd::prelude::*;

fn main() {}
verus! {

fn array_all_positive(a: &Vec<i32>) -> (result: bool)
    ensures
        result <==> forall|i: int| 0 <= i < a.len() ==> a[i] > 0,
{
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            forall|i: int| 0 <= i < n ==> a[i] > 0,
        decreases a.len() - n,
    {
        if a[n] <= 0 {
            return false;
        }
        n = n + 1;
    }
    true
}

} // verus!
