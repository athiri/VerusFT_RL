use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_last_push<A>(s: Seq<A>, a: A)
    requires
        s.len() > 0,
    ensures
        s.push(a).last() == a,
{
}

}
