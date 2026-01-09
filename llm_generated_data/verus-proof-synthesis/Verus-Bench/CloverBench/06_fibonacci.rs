use vstd::prelude::*;

fn main() {}
verus! {

fn fibonacci_10() -> (result: u32)
    ensures
        result == 55,
{
    let mut prev: u32 = 0;
    let mut curr: u32 = 1;
    let mut i: u32 = 2;
    while i <= 10
        invariant
            2 <= i <= 11,
            i == 2 ==> prev == 0 && curr == 1,
            i == 3 ==> prev == 1 && curr == 1,
            i == 4 ==> prev == 1 && curr == 2,
            i == 5 ==> prev == 2 && curr == 3,
            i == 6 ==> prev == 3 && curr == 5,
            i == 7 ==> prev == 5 && curr == 8,
            i == 8 ==> prev == 8 && curr == 13,
            i == 9 ==> prev == 13 && curr == 21,
            i == 10 ==> prev == 21 && curr == 34,
            i == 11 ==> prev == 34 && curr == 55,
        decreases 11 - i,
    {
        let next = prev + curr;
        prev = curr;
        curr = next;
        i = i + 1;
    }
    curr
}

} // verus!
