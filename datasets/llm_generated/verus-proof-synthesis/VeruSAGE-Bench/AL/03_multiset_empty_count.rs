use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

proof fn multiset_empty_len<V>()
    ensures
        Multiset::<V>::empty().len() == 0,
{
}

}
