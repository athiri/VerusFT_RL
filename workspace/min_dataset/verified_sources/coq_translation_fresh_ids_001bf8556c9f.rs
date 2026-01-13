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

} // verus!