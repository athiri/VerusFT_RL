use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s.first() == v {
        1 + count(s.drop_first(), v)
    } else {
        count(s.drop_first(), v)
    }
}

pub proof fn count_singleton(x: nat, v: nat)
    ensures count(seq![x], v) == if x == v { 1nat } else { 0nat }
{
    reveal_with_fuel(count, 2);
}

} // verus!
