use vstd::prelude::*;

fn main() {}

verus! {

proof fn set_intersect_empty<T>(s: Set<T>)
    ensures
        s.intersect(Set::<T>::empty()) == Set::<T>::empty(),
{
}

}
