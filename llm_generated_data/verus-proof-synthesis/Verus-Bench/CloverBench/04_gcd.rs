use vstd::prelude::*;

fn main() {}
verus! {

fn gcd(a: u32, b: u32) -> (result: u32)
    requires
        a > 0,
    ensures
        result > 0,
{
    let mut a = a;
    let mut b = b;
    while b != 0
        invariant
            a > 0,
        decreases b,
    {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

} // verus!
