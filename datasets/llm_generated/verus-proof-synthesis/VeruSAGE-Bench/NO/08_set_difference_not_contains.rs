use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_difference_not_contains<T>(s1: Set<T>, s2: Set<T>, x: T)
    requires
        s2.contains(x),
    ensures
        !s1.difference(s2).contains(x),
{
}

}
