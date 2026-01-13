use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn is_fresh_for_set(id: Id, used: Set<Id>) -> bool {
    !used.contains(id)
}


pub proof fn fresh_for_singleton(x: Id)
    ensures is_fresh_for_set(x + 1, Set::empty().insert(x))
{
    let s = Set::empty().insert(x);
    assert(!s.contains(x + 1));
}

} // verus!