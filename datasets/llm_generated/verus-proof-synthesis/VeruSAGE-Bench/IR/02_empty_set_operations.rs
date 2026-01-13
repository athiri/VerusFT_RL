use vstd::prelude::*;
fn main() {}
verus! {

// Empty set union is identity
pub proof fn empty_set_union<T>(s: Set<T>)
    ensures
        s.union(Set::<T>::empty()) == s,
{
}

} // verus!
