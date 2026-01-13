use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn fresh_ids(bound: Id, n: nat) -> Seq<Id>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        Seq::new(n, |i: int| bound + 1 + i as nat)
    }
}


pub proof fn fresh_ids_length(bound: Id, n: nat)
    ensures fresh_ids(bound, n).len() == n
{
}

} // verus!