use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_remove_not_contains<T>(s: Set<T>, x: T)
    ensures
        !s.remove(x).contains(x),
{
}

}
