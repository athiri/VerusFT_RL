use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_push_subrange<A>(s: Seq<A>, x: A)
    ensures
        s.push(x).subrange(0, s.len() as int) == s,
{
}

}
