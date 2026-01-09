use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_add_len<A>(s1: Seq<A>, s2: Seq<A>)
    ensures
        (s1 + s2).len() == s1.len() + s2.len(),
{
}

}
