use vstd::prelude::*;

verus! {

pub open spec fn insert(x: nat, s: Seq<nat>) -> Seq<nat>
    decreases s.len()
{
    if s.len() == 0 {
        seq![x]
    } else if x <= s[0] {
        seq![x].add(s)
    } else {
        seq![s[0]].add(insert(x, s.skip(1)))
    }
}


pub proof fn insert_length(x: nat, s: Seq<nat>)
    ensures insert(x, s).len() == s.len() + 1
    decreases s.len()
{
    reveal_with_fuel(insert, 2);
    if s.len() == 0 {
    } else if x <= s[0] {
    } else {
        insert_length(x, s.skip(1));
    }
}

} // verus!