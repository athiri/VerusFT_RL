use vstd::prelude::*;
fn main() {}
verus! {

// Empty map has no keys
pub proof fn map_empty_no_keys<K, V>()
    ensures
        Map::<K, V>::empty().dom() == Set::<K>::empty(),
{
}

} // verus!
