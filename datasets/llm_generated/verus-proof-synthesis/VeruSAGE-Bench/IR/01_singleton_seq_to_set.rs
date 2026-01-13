use vstd::prelude::*;
fn main() {}
verus! {

// Simple proof about empty sequence
pub proof fn empty_seq_to_set<T>()
    ensures
        Seq::<T>::empty().to_set() == Set::<T>::empty(),
{
}

} // verus!
