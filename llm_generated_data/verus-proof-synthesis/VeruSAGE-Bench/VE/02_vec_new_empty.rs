use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_empty_len<A>()
    ensures
        Seq::<A>::empty().len() == 0,
{
}

}
