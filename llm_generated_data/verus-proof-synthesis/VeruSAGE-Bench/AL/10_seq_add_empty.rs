use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_add_empty<A>(s: Seq<A>)
    ensures
        s + Seq::<A>::empty() == s,
{
}

}
