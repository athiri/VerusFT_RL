use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_union_self<T>(s: Set<T>)
    ensures
        s.union(s) == s,
{
}

}
