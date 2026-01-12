use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_push_len<A>(s: Seq<A>, a: A)
    ensures
        s.push(a).len() == s.len() + 1,
{
}

}
