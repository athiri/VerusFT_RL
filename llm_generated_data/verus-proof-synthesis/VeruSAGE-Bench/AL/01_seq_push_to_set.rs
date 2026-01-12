use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_push_len<A>(s: Seq<A>, e: A)
    ensures
        s.push(e).len() == s.len() + 1,
{
}

}
