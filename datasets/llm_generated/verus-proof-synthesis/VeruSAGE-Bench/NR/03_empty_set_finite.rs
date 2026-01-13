use vstd::prelude::*;

fn main() {}

verus! {

proof fn empty_set_finite<T>()
    ensures
        Set::<T>::empty().finite(),
{
}

}
