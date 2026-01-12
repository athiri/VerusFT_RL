use vstd::prelude::*;
fn main() {}
verus! {

// Removing element means it's not contained
pub proof fn set_remove_not_contains<T>(s: Set<T>, x: T)
    ensures
        !s.remove(x).contains(x),
{
}

} // verus!
