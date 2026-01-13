use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_push_len<A>(s: Seq<A>, x: A)
    ensures
        s.push(x).len() == s.len() + 1,
{
}

}
