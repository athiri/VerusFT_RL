use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_empty_dom<K, V>()
    ensures
        Map::<K, V>::empty().dom().len() == 0,
{
}

}
