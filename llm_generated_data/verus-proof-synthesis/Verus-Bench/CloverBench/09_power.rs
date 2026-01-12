use vstd::prelude::*;

fn main() {}
verus! {

fn compute_2_power_5() -> (result: u32)
    ensures
        result == 32,
{
    let mut result: u32 = 1;
    let mut i: u32 = 0;
    while i < 5
        invariant
            i <= 5,
            i == 0 ==> result == 1,
            i == 1 ==> result == 2,
            i == 2 ==> result == 4,
            i == 3 ==> result == 8,
            i == 4 ==> result == 16,
            i == 5 ==> result == 32,
        decreases 5 - i,
    {
        result = result * 2;
        i = i + 1;
    }
    result
}

} // verus!
