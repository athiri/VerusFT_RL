use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_empty_to_set<A>()
    ensures
        Seq::<A>::empty().to_set() == Set::<A>::empty(),
{
}

}
