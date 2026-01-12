use vstd::prelude::*;

fn main() {}

verus! {

proof fn empty_set_len<T>()
    ensures
        Set::<T>::empty().len() == 0,
{
}

}
