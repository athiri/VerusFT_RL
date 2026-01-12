use vstd::prelude::*;
fn main() {}

verus! {

fn filter_even(input: &Vec<u64>, output: &mut Vec<u64>)
    requires
        old(output).len() == 0,
    ensures
        output@ == input@.filter(|k: u64| k % 2 == 0),
{
    let mut i: usize = 0;
    let len = input.len();

    assert(output@ == input@.take(0).filter(|k: u64| k % 2 == 0));

    while i < len
        invariant
            0 <= i <= len,
            input@.len() == len,
            output@ == input@.take(i as int).filter(|k: u64| k % 2 == 0),
        decreases len - i,
    {
        if input[i] % 2 == 0 {
            output.push(input[i]);
        }
        assert(input@.take((i + 1) as int).drop_last() == input@.take(i as int));
        reveal(Seq::filter);
        i = i + 1;
    }
    assert(input@ == input@.take(input.len() as int));
}

} // verus!
