use vstd::prelude::*;

fn main() {}
verus! {

fn is_sorted(a: &Vec<i32>) -> (result: bool)
    ensures
        result <==> forall|i: int, j: int| 0 <= i < j < a.len() ==> a[i] <= a[j],
{
    if a.len() <= 1 {
        return true;
    }
    let mut n: usize = 1;
    while n < a.len()
        invariant
            1 <= n <= a.len(),
            forall|i: int, j: int| 0 <= i < j < n ==> a[i] <= a[j],
        decreases a.len() - n,
    {
        if a[n - 1] > a[n] {
            return false;
        }
        n = n + 1;
    }
    true
}

} // verus!
