use vstd::prelude::*;

verus! {

pub open spec fn is_even(n: nat) -> bool
    decreases n
{
    if n == 0 {
        true
    } else if n == 1 {
        false
    } else {
        is_even((n - 2) as nat)
    }
}


pub proof fn lemma_is_even_6()
    ensures is_even(6)
{
    reveal_with_fuel(is_even, 4);
    assert(is_even(6));
}

} // verus!