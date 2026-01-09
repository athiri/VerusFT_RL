use vstd::prelude::*;
fn main() {}
verus! {

// Set union is commutative
pub proof fn set_union_commutative<T>(s: Set<T>, t: Set<T>)
    ensures
        s.union(t) == t.union(s),
{
}

} // verus!
