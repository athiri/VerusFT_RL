use vstd::prelude::*;

fn main() {}
verus! {

fn is_even(n: u32) -> (result: bool)
    ensures
        result <==> n % 2 == 0,
{
    n % 2 == 0
}

fn count_even(a: &Vec<u32>) -> (count: usize)
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
        if is_even(a[n]) {
            count = count + 1;
        }
        n = n + 1;
    }
    count
}

} // verus!
