use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_union_finite<T>(s1: Set<T>, s2: Set<T>)
    requires
        s1.finite(),
        s2.finite(),
    ensures
        s1.union(s2).finite(),
{
}

}
