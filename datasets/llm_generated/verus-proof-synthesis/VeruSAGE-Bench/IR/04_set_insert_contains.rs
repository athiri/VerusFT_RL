use vstd::prelude::*;
fn main() {}
verus! {

// Set insert makes element contained
pub proof fn set_insert_contains<T>(s: Set<T>, x: T)
    ensures
        s.insert(x).contains(x),
{
}

} // verus!
