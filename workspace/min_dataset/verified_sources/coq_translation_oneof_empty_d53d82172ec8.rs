use vstd::prelude::*;

verus! {

pub open spec fn oneof_outputs<T>(gens: Seq<Set<T>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int| 0 <= i < gens.len() && gens[i].contains(x)
    )
}


pub proof fn oneof_empty<T>()
    ensures oneof_outputs::<T>(Seq::empty()) =~= Set::empty()
{
    assert forall|x: T| !oneof_outputs::<T>(Seq::empty()).contains(x) by {}
}

} // verus!