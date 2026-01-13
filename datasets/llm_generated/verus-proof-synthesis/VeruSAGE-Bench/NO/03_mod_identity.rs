use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_intersect_subset<T>(s1: Set<T>, s2: Set<T>)
    ensures
        s1.intersect(s2).subset_of(s1),
{
}

}
