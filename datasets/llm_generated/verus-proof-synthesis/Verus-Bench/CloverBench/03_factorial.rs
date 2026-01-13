use vstd::prelude::*;

fn main() {}
verus! {

fn compute_factorial_5() -> (result: u32)
    ensures
        result == 120,
{
    let mut result: u32 = 1;
    let mut i: u32 = 1;
    while i <= 5
        invariant
            1 <= i <= 6,
            i == 1 ==> result == 1,
            i == 2 ==> result == 1,
            i == 3 ==> result == 2,
            i == 4 ==> result == 6,
            i == 5 ==> result == 24,
            i == 6 ==> result == 120,
        decreases 6 - i,
    {
        result = result * i;
        i = i + 1;
    }
    result
}

} // verus!
