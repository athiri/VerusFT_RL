use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_empty_identity<A>(s: Seq<A>)
    ensures
        Seq::<A>::empty() + s == s,
{
}

}
