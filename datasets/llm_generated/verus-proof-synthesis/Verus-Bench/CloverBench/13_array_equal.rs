use vstd::prelude::*;

fn main() {}
verus! {

fn array_equal(a: &Vec<i32>, b: &Vec<i32>) -> (result: bool)
    ensures
        result <==> (a.len() == b.len() && forall|i: int| 0 <= i < a.len() ==> a[i] == b[i]),
{
    if a.len() != b.len() {
        return false;
    }
    let mut n: usize = 0;
    while n < a.len()
        invariant
            n <= a.len(),
            a.len() == b.len(),
            forall|i: int| 0 <= i < n ==> a[i] == b[i],
        decreases a.len() - n,
    {
        if a[n] != b[n] {
            return false;
        }
        n = n + 1;
    }
    true
}

} // verus!
