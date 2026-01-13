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


pub proof fn fresh_ids_all_fresh(bound: Id, n: nat)
    ensures forall|i: int| 0 <= i < n as int ==> (#[trigger] fresh_ids(bound, n)[i]) > bound
{
    if n > 0 {
        assert forall|i: int| 0 <= i < n as int implies (#[trigger] fresh_ids(bound, n)[i]) > bound by {
            assert(fresh_ids(bound, n)[i] == bound + 1 + i as nat);
        }
    }
}

} // verus!