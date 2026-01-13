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


pub proof fn fresh_ids_distinct(bound: Id, n: nat)
    ensures forall|i: int, j: int|
        0 <= i < n as int && 0 <= j < n as int && i != j
        ==> (#[trigger] fresh_ids(bound, n)[i]) != (#[trigger] fresh_ids(bound, n)[j])
{
    if n > 0 {
        assert forall|i: int, j: int|
            0 <= i < n as int && 0 <= j < n as int && i != j
            implies (#[trigger] fresh_ids(bound, n)[i]) != (#[trigger] fresh_ids(bound, n)[j]) by {
            assert(fresh_ids(bound, n)[i] == bound + 1 + i as nat);
            assert(fresh_ids(bound, n)[j] == bound + 1 + j as nat);
        }
    }
}

} // verus!