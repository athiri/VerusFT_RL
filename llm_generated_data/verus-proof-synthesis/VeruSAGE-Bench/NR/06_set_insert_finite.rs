use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_insert_finite<T>(s: Set<T>, x: T)
    requires
        s.finite(),
    ensures
        s.insert(x).finite(),
{
}

}
