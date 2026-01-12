use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_add_associative<A>(s1: Seq<A>, s2: Seq<A>, s3: Seq<A>)
    ensures
        (s1 + s2) + s3 == s1 + (s2 + s3),
{
}

}
